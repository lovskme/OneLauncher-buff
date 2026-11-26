use bytes::Bytes;
use freya::query::{Query, QueryCapability, use_query};
use oneclient_core::offline_appearance::{self, Appearance};
use uuid::Uuid;
use freya::prelude::*;

use crate::AppAssets;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct LocalAppearanceQuery;

impl QueryCapability for LocalAppearanceQuery {
    type Ok = Appearance;
    type Err = String;
    type Keys = Option<Uuid>;

    async fn run(&self, keys: &Self::Keys) -> Result<Appearance, String> {
        match keys {
            Some(id) => offline_appearance::load(*id).await,
            None => Ok(Appearance::default()),
        }
    }
}

pub fn use_local_appearance(uuid: &str) -> Appearance {
    let id = Uuid::parse_str(uuid).ok();
    let query = use_query(Query::new(id, LocalAppearanceQuery));
    let dir = id.and_then(|id| offline_appearance::profile_dir(id).ok());
    super::folder_watch::use_folder_watch(dir, false, |_, event| {
        event.paths.iter().any(|path| path.file_name().is_some_and(|name|
            name == "skin.png" || name == "cape.png" || name == "model.txt"))
    }, move || query.invalidate());
    super::settled_or_loading(&query).unwrap_or_default()
}

pub fn use_player_skin(uuid: String) -> (Bytes, bool) {
    let local = use_local_appearance(&uuid);
    let profile = super::use_player_profile(uuid.clone(), None::<String>);

    let (skin_url, is_slim) = crate::hooks::settled_or_loading(&profile)
        .map_or((None, false), |profile| (profile.skin_url, profile.is_slim));

    let skin_query = super::use_cached_image(skin_url.clone(), 256);

    let steve = use_memo(|| AppAssets::get_bytes("steve.png").unwrap_or_default());
    let alex = use_memo(|| AppAssets::get_bytes("alex.png").unwrap_or_default());

    let default_slim = (java_string_hash(&uuid) & 1) == 1;

    if let Some(skin) = local.skin { return (skin.into(), local.slim); }

    match crate::hooks::loaded_image(skin_url.as_deref(), &skin_query) {
        Some((_, bytes)) => (bytes, is_slim),
        None if default_slim => (alex.read().clone(), true),
        None => (steve.read().clone(), false),
    }
}

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
