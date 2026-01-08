use crate::component::Component;
use runa_asset::assets::ImageAsset;
use runa_asset::handle::Handle;

pub struct SpriteRenderer {
    pub texture: Handle<TextureAsset>,
}

impl Component for SpriteRenderer {
    fn start(&mut self, _ctx: &mut std::task::Context) {
        let image = runa_asset::loader::load_image("../../../assets/player.png");
    }
}
