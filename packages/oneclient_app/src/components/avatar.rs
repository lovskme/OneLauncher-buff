use std::hash::{Hash, Hasher};
use freya::elements::image::image;
use freya::prelude::*;

use crate::hooks::use_player_skin;
use super::avatar_cache::use_avatar_head;
use crate::theme::rounded;

const AVATAR_SIZE: f32 = 32.;
const FACE: f32 = 8.;

#[derive(PartialEq, Clone)]
pub struct Avatar {
    uuid: String,
    size: f32,
    radius: Option<f32>,
    layout: LayoutData,
}

impl Avatar {
    pub fn new(uuid: impl Into<String>) -> Self {
        Self {
            uuid: uuid.into(),
            size: AVATAR_SIZE,
            radius: None,
            layout: LayoutData::default(),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }
}

impl LayoutExt for Avatar {
    fn layout_mut(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for Avatar {}

impl Component for Avatar {
    fn render_key(&self) -> DiffKey {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.uuid.hash(&mut hasher);
        DiffKey::U64(hasher.finish())
    }

    fn render(&self) -> impl IntoElement {
        let (skin_bytes, _is_slim) = use_player_skin(self.uuid.clone());

        let scale_factor = *Platform::get().current_window().scale_factor.read() as f32;
        let texel = texel_px(self.size, scale_factor);
        let face = face_size(texel, scale_factor);

        let head = use_avatar_head(skin_bytes, texel);

        let radius = rounded(self.radius.unwrap_or(self.size / 4.));
        let mut nudge = use_state(|| (0f32, 0f32));
        let inset = (self.size - face) / 2.;
        let (dx, dy) = *nudge.read();

        rect()
            .width(Size::px(self.size))
            .height(Size::px(self.size))
            .corner_radius(radius)
            .on_sized(move |e: Event<SizedEventData>| {
                let snap = |at: f32| {
                    let physical = (at + inset) * scale_or_one(scale_factor);
                    (physical.round() - physical) / scale_or_one(scale_factor)
                };
                nudge.set_if_modified((snap(e.area.min_x()), snap(e.area.min_y())));
            })
            .maybe_child(head.map(|holder| {
                image(holder)
                    .position(Position::new_absolute().left(inset + dx).top(inset + dy))
                    .width(Size::px(face))
                    .height(Size::px(face))
                    .aspect_ratio(AspectRatio::None)
                    .sampling_mode(SamplingMode::Nearest)
                    .corner_radius(radius)
            }))
    }
}

fn scale_or_one(scale_factor: f32) -> f32 {
    if scale_factor > 0. { scale_factor } else { 1. }
}

fn texel_px(size: f32, scale_factor: f32) -> u32 {
    (size * scale_or_one(scale_factor) / FACE).floor().max(1.) as u32
}

fn face_size(texel: u32, scale_factor: f32) -> f32 {
    texel as f32 * FACE / scale_or_one(scale_factor)
}


#[cfg(test)]
mod tests {
    use super::{face_size, texel_px};

    #[test]
    fn faces_snap_down_to_whole_texels() {
        assert_eq!(texel_px(28., 1.), 3);
        assert_eq!(face_size(3, 1.), 24.);
        assert_eq!(texel_px(32., 1.), 4);
        assert_eq!(texel_px(16., 1.), 2);
    }

    #[test]
    fn snapping_follows_the_display_scale() {
        assert_eq!(texel_px(28., 1.25), 4);
        assert_eq!(face_size(4, 1.25), 25.6);
        assert_eq!(texel_px(32., 1.5), 6);
    }
}
