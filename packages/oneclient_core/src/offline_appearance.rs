//! Per-profile local textures shared with the bundled 1.8.9 wardrobe.
use std::io::Cursor;
use std::path::{Path, PathBuf};

use oneclient_common::paths;
use oneclient_net::RequestClient;
use uuid::Uuid;

const MAX_PNG: usize = 1024 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Appearance {
    pub skin: Option<Vec<u8>>,
    pub cape: Option<Vec<u8>>,
    pub slim: bool,
}

pub fn profile_dir(account: Uuid) -> Result<PathBuf, String> {
    Ok(paths::data_dir().map_err(|e| e.to_string())?
        .join("offline-appearance").join(account.to_string()))
}

pub fn validate_png(bytes: &[u8], cape: bool) -> Result<(), String> {
    if bytes.len() > MAX_PNG {
        return Err("PNG files must be smaller than 1 MB.".into());
    }
    let dimensions = image::ImageReader::with_format(Cursor::new(bytes), image::ImageFormat::Png)
        .into_dimensions().map_err(|e| format!("Invalid PNG: {e}"))?;
    let valid = if cape { dimensions == (64, 32) } else {
        matches!(dimensions, (64, 32) | (64, 64))
    };
    if !valid {
        return Err(if cape { "Capes must be 64 x 32 PNG images." } else {
            "Skins must be 64 x 64 or 64 x 32 PNG images."
        }.into());
    }
    image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
        .map_err(|e| format!("Invalid PNG: {e}"))?;
    Ok(())
}

async fn optional_file(path: PathBuf) -> Result<Option<Vec<u8>>, String> {
    match tokio::fs::read(path).await {
        Ok(bytes) => Ok(Some(bytes)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn load(account: Uuid) -> Result<Appearance, String> {
    let dir = profile_dir(account)?;
    Ok(Appearance {
        skin: optional_file(dir.join("skin.png")).await?,
        cape: optional_file(dir.join("cape.png")).await?,
        slim: optional_file(dir.join("model.txt")).await?
            .is_some_and(|bytes| bytes == b"slim"),
    })
}

pub async fn save_png(account: Uuid, bytes: &[u8], cape: bool, slim: bool) -> Result<(), String> {
    validate_png(bytes, cape)?;
    let dir = profile_dir(account)?;
    if !cape {
        polyio::write_atomic(dir.join("model.txt"), if slim { b"slim".as_slice() } else { b"classic".as_slice() })
            .await.map_err(|e| e.to_string())?;
    }
    polyio::write_atomic(dir.join(if cape { "cape.png" } else { "skin.png" }), bytes)
        .await.map_err(|e| e.to_string())
}

pub async fn import_png(account: Uuid, path: &Path, cape: bool, slim: bool) -> Result<(), String> {
    if tokio::fs::metadata(path).await.map_err(|e| e.to_string())?.len() > MAX_PNG as u64 {
        return Err("PNG files must be smaller than 1 MB.".into());
    }
    let bytes = tokio::fs::read(path).await.map_err(|e| e.to_string())?;
    save_png(account, &bytes, cape, slim).await
}

pub async fn clear(account: Uuid, cape: bool) -> Result<(), String> {
    let path = profile_dir(account)?.join(if cape { "cape.png" } else { "skin.png" });
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn from_username(client: &RequestClient, account: Uuid, name: &str) -> Result<(), String> {
    let (id, _) = oneclient_mc::resolve_username(client, name).await.map_err(|e| e.to_string())?;
    let profile = oneclient_mc::fetch_player_profile(client, &id).await.map_err(|e| e.to_string())?;
    let source = profile.skin_url.ok_or("This player has no custom skin. Choose a PNG instead.")?;
    let mut url = reqwest::Url::parse(&source).map_err(|e| e.to_string())?;
    if url.host_str() != Some("textures.minecraft.net") {
        return Err("Unexpected Minecraft texture host.".into());
    }
    url.set_scheme("https").map_err(|_| "Invalid texture URL")?;
    let mut response = client.send(reqwest::Request::new(reqwest::Method::GET, url))
        .await.map_err(|e| e.to_string())?.error_for_status().map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        if bytes.len() + chunk.len() > MAX_PNG { return Err("Skin download exceeds 1 MB.".into()); }
        bytes.extend_from_slice(&chunk);
    }
    save_png(account, &bytes, false, profile.is_slim).await
}

/// Resolves the directory the loader actually receives, including a user override.
pub fn active_mods_dir(cwd: &Path, jvm_args: &[String]) -> PathBuf {
    jvm_args.iter().rev()
        .find_map(|arg| arg.strip_prefix("-Dfabric.modsFolder="))
        .map(|value| cwd.join(value))
        .unwrap_or_else(|| cwd.join("mods"))
}

/// Replaces only materialized PolyPlus copies, preserving the shared artifact cache.
#[cfg(feature = "local-appearance")]
pub async fn prepare_game(mods: &Path) -> Result<(), String> {
    const JAR: &[u8] = include_bytes!("../assets/local-appearance-1.8.9.jar");
    let mut folders = vec![(mods.to_path_buf(), 0)];
    let mut replaced = 0;
    while let Some((folder, depth)) = folders.pop() {
        let mut entries = tokio::fs::read_dir(&folder).await
            .map_err(|e| format!("Could not inspect active mods folder {}: {e}", folder.display()))?;
        while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
            let path = entry.path();
            // Older loader layouts put the cluster link one level below mods.
            if depth == 0 && path.is_dir() {
                folders.push((path, 1));
            } else if path.extension().is_some_and(|ext| ext == "jar") {
                let manifest = oneclient_content::packages::local_manifest::read_jar_manifest(&path).await;
                if manifest.mod_id.as_deref() == Some("polyplus") {
                    polyio::write_atomic(&path, JAR).await.map_err(|e| e.to_string())?;
                    replaced += 1;
                    tracing::info!(path = %path.display(), "replaced PolyPlus with local appearance build");
                }
            }
        }
    }
    if replaced == 0 {
        return Err(format!("Local appearance component was not found in {}. Repair the OneClient installation before launching.", mods.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn png(width: u32, height: u32) -> Vec<u8> {
        let mut out = Cursor::new(Vec::new());
        image::DynamicImage::new_rgba8(width, height).write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }
    #[test]
    fn active_mods_folder_follows_final_jvm_property() {
        let cwd = Path::new("game");
        assert_eq!(active_mods_dir(cwd, &[]), cwd.join("mods"));
        let args = vec!["-Dfabric.modsFolder=old".into(), "-Xmx2G".into(),
            "-Dfabric.modsFolder=cluster with spaces/mods".into()];
        assert_eq!(active_mods_dir(cwd, &args), cwd.join("cluster with spaces/mods"));
    }

    #[test]
    fn validates_skin_and_cape_dimensions_and_encoding() {
        assert!(validate_png(&png(64, 64), false).is_ok());
        assert!(validate_png(&png(64, 32), false).is_ok());
        assert!(validate_png(&png(64, 32), true).is_ok());
        assert!(validate_png(&png(64, 64), true).is_err());
        assert!(validate_png(&png(32, 32), false).is_err());
        assert!(validate_png(b"not a PNG", false).is_err());
        assert!(validate_png(&vec![0; MAX_PNG + 1], false).is_err());
    }
}
