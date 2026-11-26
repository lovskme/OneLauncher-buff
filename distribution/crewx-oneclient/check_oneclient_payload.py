"""Static linkage gate. Never imports or executes the payload or Minecraft classes."""
import argparse
import re
import struct
import zipfile
from collections import deque
from pathlib import Path


def read_class(data):
    if data[:4] != b'\xca\xfe\xba\xbe':
        raise ValueError('Not a class file')
    pos = 8

    def number(fmt):
        nonlocal pos
        result = struct.unpack_from(fmt, data, pos)[0]
        pos += struct.calcsize(fmt)
        return result

    cp = [None] * number('>H')
    i = 1
    while i < len(cp):
        tag = number('>B')
        if tag == 1:
            length = number('>H')
            cp[i] = data[pos:pos + length].decode('utf8', errors='replace')
            pos += length
        elif tag in (3, 4):
            pos += 4
        elif tag in (5, 6):
            pos += 8
            i += 1
        elif tag in (7, 8, 16, 19, 20):
            cp[i] = (tag, number('>H'))
        elif tag in (9, 10, 11, 12, 17, 18):
            cp[i] = (tag, number('>H'), number('>H'))
        elif tag == 15:
            pos += 3
        else:
            raise ValueError('Unknown constant pool tag: ' + str(tag))
        i += 1

    def class_name(index):
        return cp[cp[index][1]] if index else None

    number('>H')
    name = class_name(number('>H'))
    parent = class_name(number('>H'))
    parents = ([parent] if parent else []) + [class_name(number('>H'))
                                             for _ in range(number('>H'))]

    def attributes():
        nonlocal pos
        for _ in range(number('>H')):
            number('>H')
            length = number('>I')
            pos += length

    def members():
        result = set()
        for _ in range(number('>H')):
            number('>H')
            member = cp[number('>H')]
            descriptor = cp[number('>H')]
            result.add((member, descriptor))
            attributes()
        return result

    fields = members()
    methods = members()
    refs = set()
    for value in cp:
        if isinstance(value, tuple) and value[0] in (9, 10, 11):
            nt = cp[value[2]]
            refs.add((value[0] == 9, class_name(value[1]), cp[nt[1]], cp[nt[2]]))
    return dict(name=name, parents=parents, fields=fields, methods=methods, refs=refs)


def index_jar(path):
    with zipfile.ZipFile(path) as archive:
        classes = [read_class(archive.read(name)) for name in archive.namelist()
                   if name.endswith('.class')]
    return {c['name']: c for c in classes}


def entries(path):
    return [line.split() for line in Path(path).read_text().splitlines()]


def mapped_host(game, resources):
    """Compose official game declarations with the same MCP -> Calamus snapshot."""
    fixed = entries(resources / 'ornithe-joined.srg')
    by_named = {tuple(p[2:]) if p[0] == 'CL:' else tuple(p[3:])
                if p[0] == 'MD:' else tuple(p[2:]): p for p in fixed if p}
    classes, fields, methods = {}, {}, {}
    for p in entries(resources / 'notch-joined.srg'):
        if not p or p[0] not in ('CL:', 'FD:', 'MD:'):
            continue
        target = by_named.get(tuple(p[3:]) if p[0] == 'MD:' else tuple(p[2:]))
        if not target:
            continue
        if p[0] == 'CL:':
            classes[p[1]] = target[1]
        elif p[0] == 'FD:':
            fields[p[1]] = target[1].rsplit('/', 1)[1]
        else:
            methods[(p[1], p[2])] = target[1].rsplit('/', 1)[1]

    def desc(value):
        return re.sub(r'L([^;]+);', lambda m: 'L' + classes.get(m[1], m[1]) + ';', value)

    def member(owner, name, descriptor, field):
        queue, seen = deque([owner]), set()
        while queue:
            current = queue.popleft()
            if current in seen:
                continue
            seen.add(current)
            key = current + '/' + name
            value = fields.get(key) if field else methods.get((key, descriptor))
            if value:
                return value
            queue.extend(game.get(current, {}).get('parents', []))
        return name

    return {classes.get(name, name): dict(
        name=classes.get(name, name),
        parents=[classes.get(p, p) for p in c['parents']],
        fields={(member(name, n, d, True), desc(d)) for n, d in c['fields']},
        methods={(member(name, n, d, False), desc(d)) for n, d in c['methods']})
        for name, c in game.items()}


def resolve(host, owner, member, descriptor, field):
    pending, seen = deque([owner]), set()
    java_methods = {('toString', '()Ljava/lang/String;'), ('hashCode', '()I'),
                    ('equals', '(Ljava/lang/Object;)Z'), ('getClass', '()Ljava/lang/Class;'),
                    ('clone', '()Ljava/lang/Object;'), ('finalize', '()V')}
    if not field and (member, descriptor) in java_methods:
        return True
    while pending:
        current = pending.popleft()
        if current in seen:
            continue
        seen.add(current)
        if not field and current == 'java/lang/Object' and (member, descriptor) in java_methods:
            return True
        if not field and current == 'java/lang/Thread' and (member, descriptor) in {
                ('start', '()V'), ('interrupt', '()V')}:
            return True
        if not field and current == 'java/lang/Enum' and (member, descriptor) in {
                ('ordinal', '()I'), ('name', '()Ljava/lang/String;')}:
            return True
        node = host.get(current)
        if node:
            if (member, descriptor) in node['fields' if field else 'methods']:
                return True
            pending.extend(node['parents'])
    return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('payload', type=Path)
    parser.add_argument('official_game', type=Path)
    parser.add_argument('resources', type=Path)
    parser.add_argument('--minecraft-dump', type=Path)
    parser.add_argument('--report', type=Path)
    args = parser.parse_args()
    host = mapped_host(index_jar(args.official_game), args.resources)
    if args.minecraft_dump:
        actual = read_class(args.minecraft_dump.read_bytes())
        host[actual['name']] = actual
    payload = index_jar(args.payload)
    combined = dict(host, **payload)
    failures, checked = set(), 0
    # These are Forge-only extensions, guarded by ForgeCompat in the source.
    forge_only = {('crewx/module/modules/combat/HitBox', 'canRiderInteract'),
                  ('crewx/module/modules/combat/Piercing', 'canRiderInteract'),
                  ('crewx/util/ItemUtil', 'getToolClasses')}
    for name, node in payload.items():
        if not name.startswith('crewx/') or name.startswith(('crewx/mixin/', 'crewx/inject/deps/')):
            continue
        for field, owner, member, descriptor in node['refs']:
            if not owner.startswith('net/minecraft/'):
                continue
            if (name, member) in forge_only:
                continue
            checked += 1
            if not resolve(combined, owner, member, descriptor, field):
                failures.add(f'{name} -> {owner}/{member} {descriptor}')
    output = '\n'.join(sorted(failures))
    if args.report:
        args.report.write_text(output + '\n')
    print(f'OneClient linkage: {checked} references; {len(failures)} unresolved')
    print(output)
    if failures:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
