
use sprite::sprite_data::SpriteData;

pub trait Drawable {

    fn to_sprite_data(&self) -> SpriteData;
}
