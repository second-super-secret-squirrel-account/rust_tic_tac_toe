use tic_tac_toe_game::cursor::Cursor;
use game_io::game_screen::sprite::sprite_data::SpriteData;
use game_io::game_screen::sprite::sprite_data_traits::Drawable;

impl Drawable for Cursor {
    fn to_sprite_data(&self) -> SpriteData {
        let mut sprite_data = SpriteData::new();

        sprite_data.add_line("**********");
        sprite_data.add_line("*        *");
        sprite_data.add_line("*        *");
        sprite_data.add_line("*        *");
        sprite_data.add_line("*        *");
        sprite_data.add_line("*        *");
        sprite_data.add_line("**********");

        sprite_data
    }
}
