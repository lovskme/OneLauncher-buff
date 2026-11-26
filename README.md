<div align="center">

<img src=".github/media/RepoBanner.png" alt="Repository Banner" />

# OneClient  |  OneLauncher
The monorepo containing the code for OneLauncher, OneClient, and their core backend.

OneClient is a Minecraft client featuring fully 100% open-source components, offering many packaged and pre-configured mods in one click.

</div>

## Installing

You can install the latest release of OneClient from our website: [https://polyfrost.org/projects/oneclient](https://polyfrost.org/projects/oneclient)
as well as our [GitHub releases](https://github.com/Polyfrost/OneLauncher/releases/latest).

| Windows (x86_64) | macOS (Intel & Apple Silicon) | Linux (x86_64)                                          |
|------------------|-------------------------------|---------------------------------------------------------|
| Installer 🔄      | DMG 🔄                        | AppImage 🔄                                             |
|                  | App Bundle 🔄                 | DEB                                                     |
|                  |                               | RPM                                                     |
|                  |                               | *[RPM (COPR)](https://copr.fedorainfracloud.org/coprs/polyfrost/OneClient/) 🔄 |
|                  |                               | [AUR](https://aur.archlinux.org/packages/oneclient-bin)|
|                  |                               | Flatpak|

> 🔄 = Has support for autoupdating built-in

<details>

<summary><h3>Flatpak Installation</h3></summary>

On most big distros, you can just double-click the .flatpak file to install it. You can also:
1. First, navigate to the folder the .flatpak was downloaded in: (e.g. Downloads)
```
cd Downloads
```
2. You can then install the package: (change filename with the one you downloaded)
```
flatpak install ./OneClient_2.11.0_linux_x86_64.flatpak 
```
</details>
<details>

<summary><h3>COPR Installation</h3></summary>

On supported distributions (RHEL, Fedora, etc.), you can make use of the Fedora COPR instead of using the AppImage or manually installing the RPM file.
1. First, you have to enable the repository:
```
sudo dnf copr enable polyfrost/OneClient
```
2. You can then install the package:
```
sudo dnf install oneclient-bin
```

</details>

## Contributing

We welcome contributions! Please read our [contributing guidelines](CONTRIBUTING.md) before getting started.


### Requirements

The project targets **Rust 1.97** or later. You can install Rust via [rustup](https://rustup.rs/).


### Building & Running

```sh
# Run the app
cargo run -p oneclient_app

# Build a release binary
cargo build -p oneclient_app --release
```


### Offline profiles

During account setup, enter a local username and select **Add offline** to
continue without Microsoft sign-in. You can also create offline profiles under
**Settings → Accounts → Add offline**. Usernames must contain 3–16 ASCII letters,
digits, or underscores. The first profile becomes the default account.

Offline profiles can play singleplayer and join servers that allow offline
players. Servers requiring authentication and Microsoft services still require
Microsoft sign-in. Installing a game version, Java, assets, or mods can require
an internet connection; create and prepare the instance before playing without
a connection.

### Local skins and capes (this fork)

Select your offline profile and open **Wardrobe**. Import a classic/slim skin
PNG, import a cape PNG, or fetch a skin by Minecraft username. Skins must be
64x64 or 64x32 pixels, and capes must be 64x32 pixels. Name lookup needs internet;
the downloaded texture is saved locally for subsequent offline use.

The custom Windows build includes a patched OneClient/PolyPlus component for
**Minecraft 1.8.9 on Ornithe**. Create an instance with the OneClient bundle so
its normal dependencies are present. The launcher installs the component when
launching that instance with an offline profile. Open the in-game Wardrobe to
change your skin/cape there too; both screens share the same per-profile files.
Changes are visible on this client and do not sync to other players. Microsoft
profiles retain the upstream online wardrobe. Paid catalog ownership is unchanged.
This custom build does not install upstream launcher updates automatically,
because those installers would replace the local wardrobe changes.

Build with `--features local-appearance` after compiling the component using
the Windows workflow. Source changes and build details are in
`distribution/local-appearance/`.

### Packaging / Releasing

Installers are produced with [**cargo-packager**](https://github.com/crabnebula-dev/cargo-packager)
(the standalone bundler spun out of the Tauri bundler). Config lives in
[`packages/oneclient_app/Cargo.toml`](./packages/oneclient_app/Cargo.toml) under
`[package.metadata.packager]`.

```sh
cargo install cargo-packager --locked

# Build the binary, then bundle it for the current OS:
cargo build --release -p oneclient_app
cargo packager --release -p oneclient_app --formats <targets>
#   Windows: nsis      macOS: app,dmg      Linux: deb,appimage
```


### Versioning

The workspace shares a single version, defined in the root [`Cargo.toml`](./Cargo.toml) under `[workspace.package]`.

Versions and release notes come from [Knope](https://knope.tech) change files in [`.changeset/`](./.changeset).
Add one per user-facing change with `knope document-change` (installed via `cargo install knope`). The
`OneClient Release Build` workflow consumes them, bumps the version, writes [`CHANGELOG.md`](./CHANGELOG.md),
and uses the new entry as the GitHub release body, which is what the launcher's changelog page shows.


## Code signing

This program uses free code signing provided by [SignPath.io](https://signpath.io?utm_source=foundation&utm_medium=github&utm_campaign=0install), and a certificate by the [SignPath Foundation](https://signpath.org?utm_source=foundation&utm_medium=github&utm_campaign=0install). We thank them very much to their contributions to OSS software!
