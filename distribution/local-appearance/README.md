# Local appearance component

The Windows build compiles PolyPlus at commit
`462663858973b1298bc30820e707854997fcffe4`, applies `polyplus.patch`, and embeds
the resulting 1.8.9 Ornithe component in the launcher. PolyPlus is GPL-3.0 with
the additional linking exception in its `LICENSE.md`; the source is available
at https://github.com/Polyfrost/PolyPlus/tree/462663858973b1298bc30820e707854997fcffe4.
The patch is the complete corresponding source change for the bundled component.

This implements local PNG skins and capes, plus public username skin lookup.
It makes no cosmetic-store purchases and sends no forged ownership claims.
The appearance is visible on this client. Other clients do not receive it.

The launcher passes a profile-specific directory using the JVM property
`oneclient.offline.appearance.dir`. Both wardrobes share `skin.png`, `cape.png`,
and `model.txt` in that directory. Skin PNGs must be 64x64 or 64x32; capes must
be 64x32. Files are validated before decoding and saved using atomic replacement.

When launching an offline 1.8.9 Ornithe instance with PolyPlus installed, the
launcher replaces the materialized PolyPlus JAR with this component using an
atomic replacement, leaving its shared content-cache source intact.
Install the OneClient bundle first; vanilla/Forge instances do not use this
component. Online Microsoft profiles keep the upstream wardrobe behavior.

The in-game **Your skins** library uses its existing PNG, URL, username, wear,
rename, favorite, model and delete controls for offline profiles. Its metadata
is stored in `library.json` and its PNGs in `library/` beside the selected
textures. Importing a skin in the launcher also adds it to this local library
on refresh. Cape PNG controls are embedded in the existing cape drawer.
The launcher and in-game local head widgets read the selected local texture.

The installer resolves the final `-Dfabric.modsFolder` JVM argument before
replacing the component, including paths with spaces and user overrides.
Legacy layouts inspect only this cluster's subdirectory or `1.8.9`, avoiding
other instances. A missing component is a launch error instead of a silent
success. Launcher logs record the replacement path. Game logs include
`polyplus/local-appearance` when the component initializes.

Validation covers PNG decoding, local index persistence and damaged metadata,
final JVM mods-folder resolution, missing components and the nested legacy
mods layout. A successful installer build does not constitute an interactive
Minecraft gameplay test.

The launcher head cache owns and compares PNG bytes rather than comparing
allocation addresses. Query snapshots share immutable `Bytes`; cloning a
snapshot no longer allocates a different skin buffer on every redraw. The
previous combination of cloned `Vec<u8>` snapshots and pointer-based cache keys
could continuously invalidate a head widget before the home screen settled.
The isolated `oneclient_avatar_cache_tests` package compiles the production
cache module directly. Its renderer tests check bounded redraws with fresh,
identical PNG allocations and verify that changing a skin changes avatar pixels.
CI also temporarily reinstates the old pointer comparison, requires that the
redraw test fails with that fault, restores the source and reruns the fixed tests.

World rendering binds local PNGs to the active player entity by object identity.
The session UUID continues to select the local profile directory; a UUID supplied
by a multiplayer server does not change the selected local skin/cape. Other world
entities keep their original texture resolution. Tests cover server UUID changes,
a different entity with a matching UUID and the absence of a live player.
The game logs one binding entry per profile/entity UUID pair for diagnostics.
