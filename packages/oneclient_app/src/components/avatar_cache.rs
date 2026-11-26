use bytes::Bytes;
use freya::elements::image::ImageHandle;
use freya::engine::prelude::{Paint, SkData, SkImage, raster_n32_premul};
use freya::prelude::*;

const FACE: f32 = 8.;
const HEAD_FACE: (f32, f32) = (8., 8.);
const HEAD_OVERLAY: (f32, f32) = (40., 8.);

/// The cache owns its input and compares PNG contents, not allocation addresses.
/// Updating a state during render must settle on the next pass even if the
/// caller supplies a fresh allocation containing exactly the same skin.
pub(crate) fn use_avatar_head(skin_bytes: Bytes, texel: u32) -> Option<ImageHandle> {
    let mut cache = use_state(|| None::<(Bytes, u32, ImageHandle)>);
    let cached = cache.read().clone();
    match cached {
        Some((png, scale, holder)) if png == skin_bytes && scale == texel => Some(holder),
        _ => {
            let holder = compose_head(&skin_bytes, texel);
            if let Some(holder) = &holder {
                cache.set(Some((skin_bytes, texel, holder.clone())));
            }
            holder
        }
    }
}

fn compose_head(skin_bytes: &Bytes, texel: u32) -> Option<ImageHandle> {
    let data = unsafe { SkData::new_bytes(skin_bytes) };
    let skin = SkImage::from_encoded(data)?;
    let skin = skin.make_raster_image(None, None).unwrap_or(skin);

    let side = (FACE as u32 * texel) as i32;
    let mut surface = raster_n32_premul((side, side))?;
    {
        let canvas = surface.canvas();
        let paint = Paint::default();
        canvas.scale((texel as f32, texel as f32));

        canvas.draw_image(&skin, (-HEAD_FACE.0, -HEAD_FACE.1), Some(&paint));
        canvas.draw_image(&skin, (-HEAD_OVERLAY.0, -HEAD_OVERLAY.1), Some(&paint));
    }

    let head = surface.image_snapshot();
    Some(ImageHandle::new(head, skin_bytes.clone()))
}


#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::io::Cursor;
    use std::rc::Rc;

    use freya::elements::image::image;
    use freya_testing::prelude::*;

    use super::*;

    fn skin(color: [u8; 4]) -> Bytes {
        let mut pixels = image::RgbaImage::from_pixel(64, 64, image::Rgba([0, 0, 0, 0]));
        for y in 8..16 {
            for x in 8..16 {
                pixels.put_pixel(x, y, image::Rgba(color));
            }
        }
        let mut output = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(pixels)
            .write_to(&mut output, image::ImageFormat::Png).unwrap();
        output.into_inner().into()
    }

    #[derive(PartialEq)]
    struct FreshAllocation {
        png: Bytes,
        renders: Rc<Cell<usize>>,
    }

    impl Component for FreshAllocation {
        fn render(&self) -> impl IntoElement {
            let count = self.renders.get() + 1;
            self.renders.set(count);
            assert!(count <= 8, "same skin requested unbounded renders");
            // The previous query snapshots cloned Vec<u8>, giving each render
            // a different address despite containing the identical PNG.
            let fresh = Bytes::copy_from_slice(&self.png);
            let head = use_avatar_head(fresh, 4).expect("fixture head decodes");
            image(head).width(Size::px(32.)).height(Size::px(32.))
        }
    }

    #[test]
    fn fresh_png_allocations_settle_without_repainting_forever() {
        let renders = Rc::new(Cell::new(0));
        let counter = renders.clone();
        let png = skin([220, 30, 40, 255]);
        let mut test = launch_test(move || FreshAllocation {
            png: png.clone(), renders: counter.clone(),
        });
        for _ in 0..4 {
            test.sync_and_update();
        }
        assert!(renders.get() > 0 && renders.get() <= 4,
            "same skin requested unbounded renders: {} redraws", renders.get());
    }

    fn changing_skin() -> impl IntoElement {
        let stored = use_consume::<State<Bytes>>();
        let png = stored.read().clone();
        image(use_avatar_head(png, 4).expect("fixture head decodes"))
            .width(Size::px(32.)).height(Size::px(32.))
    }

    #[test]
    fn changing_the_png_updates_the_rendered_avatar_pixels() {
        let red = skin([220, 30, 40, 255]);
        let blue = skin([15, 70, 230, 255]);
        let (mut test, mut stored) = TestingRunner::new(
            changing_skin,
            Size2D::new(32., 32.),
            |runner| runner.provide_root_context(|| State::create(red)),
            1.,
        );
        test.sync_and_update();
        let snapshot = test.render();
        let pixels = image::load_from_memory(snapshot.as_bytes()).unwrap().to_rgb8();
        assert_eq!(pixels.get_pixel(16, 16).0, [220, 30, 40]);

        test.run_in(|| stored.set(blue));
        test.sync_and_update();
        let snapshot = test.render();
        let pixels = image::load_from_memory(snapshot.as_bytes()).unwrap().to_rgb8();
        assert_eq!(pixels.get_pixel(16, 16).0, [15, 70, 230]);
    }
}
