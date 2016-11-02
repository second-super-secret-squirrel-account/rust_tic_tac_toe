pub mod sprite_sheet;

use game_io::screen_builder::ScreenBuilder;
use game_io::screen_builder::point_generator;
use game_io::game_screen::sprite::Sprite;
use game_io::game_screen::GameScreen;
use game_io::game_screen::sprite::point::Point;
use game_io::game_screen::sprite::color::Color;
use tic_tac_toe_game::cursor::Cursor;

impl ScreenBuilder {
    pub fn setup_screen(&self, cursor: Cursor) -> GameScreen {
        let mut screen = self.basic_screen();
        for sprite in self.menu_list(cursor) {
            screen.add_sprite(sprite);
        }
        screen.add_sprite(self.bottom_menu_sprite());
        screen.add_sprite(self.controls_sprite());
        screen
    }

    fn bottom_menu_sprite(&self) -> Sprite {
        let point = Point::new (self.screen_height - 3, 3);
        let color = Color::YellowOnBlue;
        let data = sprite_sheet::bottom_menu();

        Sprite::new(point, color, data)

    }

    fn controls_sprite(&self) -> Sprite {
        let data = sprite_sheet::controls();
        let color = Color::WhiteOnBlue;
        let sprite_width = sprite_sheet::controls().width();
        let sprite_y = point_generator::center_side(sprite_width, self.screen_width);
        let point = Point::new (15, sprite_y);
        Sprite::new(point, color, data)
    }

    fn menu_list(&self, cursor: Cursor) -> Vec<Sprite> {
        let mut sprites = Vec::new();
        for menu_index in 0..3 {
            let menu_index_i32 = menu_index as i32;
            let data = sprite_sheet::menu_item(menu_index);
            let color = self.menu_color(menu_index_i32, cursor.clone());
            let sprite_width = data.width();
            let sprite_y = point_generator::center_side(sprite_width, self.screen_width);
            let point = Point::new(35 + menu_index_i32, sprite_y);

            sprites.push(Sprite::new(point, color, data));
        }
        sprites
    }

    fn menu_color(&self, index: i32, cursor: Cursor) -> Color {
        if index == cursor.row() {
            return Color::BlueOnYellow;
        } else {
            return Color::YellowOnBlue;
        }
    }
}
