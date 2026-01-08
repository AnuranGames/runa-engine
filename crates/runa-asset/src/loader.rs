use crate::handle::Handle;
use crate::image::ImageAsset;

pub fn load_image(path: &str) -> Handle<ImageAsset> {
    let image = ImageAsset::load(path).expect("Failed to load image");
    Handle {
        inner: std::sync::Arc::new(image),
    }
}
