pub mod sprite;
use game_io::game_screen::sprite::Sprite;
use game_io::ncurses_wrapper::output::*;

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

    pub fn draw_screen(&self) {
        clear();
        for sprite in self.sprites() {
            sprite.draw();
        }
        refresh();
    }
}
