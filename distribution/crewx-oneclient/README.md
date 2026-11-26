# CrewX for OneClient 1.8.9

This branch builds CrewX from upstream revision
`ffc30cc7ed9c7c881c397397bfbc2983bd49d3d4` plus `oneclient.patch`.
The OneClient launcher source is unchanged. The Windows build is in
`.github/workflows/crewx_oneclient.yml`.

## Changes

- Compose Calamus Gen2 and MCP stable 22 mappings with unambiguous inherited
  interface/member aliases. The old generator dropped implementations whenever
  Calamus recorded the member only on an interface or superclass.
- Pin runtime mapping resources to the payload's bundled snapshot by default.
  Explicit `crewx.mappings.document` overrides remain available; downloading
  upstream mappings now requires `-Dcrewx.mappings.remote=true`.
- Include the generated Ornithe mapping in the remapped payload.
- Require a hierarchy JAR during packaging instead of silently emitting an
  Ornithe payload without inheritance information. The CI uses Mojang's vanilla
  1.8.9 client with verified SHA-1
  `3870888a6c3d349d3771a3e9d16c9bf5e076b908`.
- Report menu key dispatch, opening requests, selected screen, initialization
  failures, and event-handler failures in `crewx-inject.log`.

## Validation

The workflow compiles and runs the mapping regression test, runs the existing
CrewX Java tests, builds the x64 Windows executable, and checks emitted payload
member references against the composed vanilla hierarchy. The checker parses
class files; it does not initialize or execute the payload.

The executable is uploaded only if every build gate passes. The tests do not
simulate OneClient's OpenGL renderer, its complete installed mod set, or a live
Minecraft session. Runtime GUI compatibility still requires testing in the actual
OneClient instance. No server protection bypass is added by this patch.

The existing default ClickGUI key remains **Insert**. Existing saved custom
binds are retained according to upstream configuration rules. If the GUI still
does not open, the new log identifies whether its bind was received, the opening
was queued, the screen changed, or a Java/linkage error occurred.

To reproduce locally on Windows, check out the pinned source, apply the patch,
copy the regression test and checker into `tools`, then follow the same generation,
test, and build commands shown in the workflow. Preserve the upstream license.
