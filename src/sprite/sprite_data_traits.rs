
use sprite::sprite_data::SpriteData;

pub trait Drawable<T> {

    fn to_sprite_data(&self) -> SpriteData;
}
