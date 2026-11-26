use freya::prelude::*;
use oneclient_core::offline_appearance::{self as local, Appearance};
use uuid::Uuid;

use crate::components::{Button, PlayerModel, ScrollArea, TextInput};
use crate::i18n::t;
use crate::theme::{colors, text_size};

#[derive(PartialEq, Hash)]
pub(super) struct OfflineWardrobe {
    pub account: Uuid,
}

#[derive(Clone)]
enum Action {
    Skin(bool),
    Cape,
    Username(String),
    Remove(bool),
}

fn run(account: Uuid, action: Action, mut appearance: State<Appearance>, mut busy: State<bool>, mut status: State<String>) {
    if *busy.peek() { return; }
    busy.set(true);
    status.set(String::new());
    spawn(async move {
        let result = match action {
            Action::Skin(_) | Action::Cape => {
                let cape = matches!(action, Action::Cape);
                let slim = if let Action::Skin(slim) = action { slim } else { false };
                match rfd::AsyncFileDialog::new().add_filter("PNG", &["png"]).pick_file().await {
                    Some(file) => local::import_png(account, file.path(), cape, slim).await,
                    None => { busy.set(false); return; }
                }
            }
            Action::Username(name) => match crate::launcher::state() {
                Ok(state) => local::from_username(&state.services.requester, account, &name).await,
                Err(err) => Err(err.to_string()),
            },
            Action::Remove(cape) => local::clear(account, cape).await,
        };
        match result {
            Ok(()) => match local::load(account).await {
                Ok(value) => {
                    appearance.set(value);
                    status.set(t("oneclient.wardrobe.local.saved").into());
                }
                Err(err) => status.set(err),
            },
            Err(err) => status.set(err),
        }
        busy.set(false);
    });
}

impl Component for OfflineWardrobe {
    fn render(&self) -> impl IntoElement {
        let account = self.account;
        let mut appearance = use_state(Appearance::default);
        let mut busy = use_state(|| true);
        let mut status = use_state(String::new);
        let name = use_state(String::new);
        use_hook(move || {
            spawn(async move {
                match local::load(account).await {
                    Ok(value) => appearance.set(value),
                    Err(err) => status.set(err),
                }
                busy.set(false);
            });
        });
        let current = appearance.read().clone();
        let enabled = !*busy.read();
        let lookup_name = name.read().trim().to_string();
        let valid_name = !lookup_name.is_empty() && lookup_name.len() <= 16
            && lookup_name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_');
        let content = rect().vertical().width(Size::fill()).padding(Gaps::new_all(24.)).spacing(16.)
            .child(label().text(t("oneclient.wardrobe.local.title")).font_size(text_size(24.)).color(colors::fg_primary()))
            .child(label().text(t("oneclient.wardrobe.local.hint")).font_size(text_size(13.)).color(colors::fg_secondary()))
            .child(PlayerModel::new(account.to_string())
                .skin(current.skin.map(|png| (png.into(), current.slim)))
                .cape(current.cape.map(Into::into))
                .height(Size::px(240.)))
            .child(rect().horizontal().content(Content::wrap_spacing(8.)).spacing(8.)
                .child(Button::new().primary().enabled(enabled).text(t("oneclient.wardrobe.local.classic"))
                    .on_press(move |_| run(account, Action::Skin(false), appearance, busy, status)))
                .child(Button::new().secondary().enabled(enabled).text(t("oneclient.wardrobe.local.slim"))
                    .on_press(move |_| run(account, Action::Skin(true), appearance, busy, status)))
                .child(Button::new().secondary().enabled(enabled).text(t("oneclient.wardrobe.local.cape"))
                    .on_press(move |_| run(account, Action::Cape, appearance, busy, status))))
            .child(TextInput::new(name).placeholder(t("oneclient.wardrobe.local.username")))
            .child(Button::new().secondary().enabled(enabled && valid_name)
                .text(t("oneclient.wardrobe.local.by_name"))
                .on_press(move |_| run(account, Action::Username(name.peek().trim().to_string()), appearance, busy, status)))
            .child(rect().horizontal().spacing(8.)
                .child(Button::new().ghost().enabled(enabled).text(t("oneclient.wardrobe.local.clear_skin"))
                    .on_press(move |_| run(account, Action::Remove(false), appearance, busy, status)))
                .child(Button::new().ghost().enabled(enabled).text(t("oneclient.wardrobe.local.clear_cape"))
                    .on_press(move |_| run(account, Action::Remove(true), appearance, busy, status))))
            .child(label().text(status.read().clone()).font_size(text_size(13.)).color(colors::fg_secondary()))
            .into_element();
        ScrollArea::new().width(Size::fill()).height(Size::fill()).child(content)
    }

    fn render_key(&self) -> DiffKey { DiffKey::U64(self.key_hash()) }
}
