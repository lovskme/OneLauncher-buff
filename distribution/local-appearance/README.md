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
