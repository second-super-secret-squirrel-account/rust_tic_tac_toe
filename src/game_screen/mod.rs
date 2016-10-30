mod screen_drawables;

use sprite::Sprite;

pub struct GameScreen {
    sprites: Vec<Sprite>,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let sprites = Vec::new();
        GameScreen {
            sprites: sprites,
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    pub fn sprites(&self) -> &Vec<Sprite> {
        &self.sprites
    }
}
