use sprite::Sprite;

pub struct GameScreen {
    sprites: Vec<Sprite<i32>>,
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let sprites = Vec::new();
        GameScreen {
            sprites: sprites,
        }
    }

    pub fn add_sprite(&mut self, sprite: Sprite<i32>) {
        self.sprites.push(sprite);
    }

    pub fn sprites(&self) -> &Vec<Sprite<i32>> {
        &self.sprites
    }
}
