use game_board::board_token::BoardToken;
use game_io::game_screen::sprite::sprite_data::SpriteData;
use game_io::game_screen::sprite::sprite_data_traits::Drawable;

impl Drawable for BoardToken {
    fn to_sprite_data(&self) -> SpriteData {
        match *self {
            BoardToken::PlayerX => self.sprite_data_player_x(),
            BoardToken::PlayerO => self.sprite_data_player_o(),
            BoardToken::Blank => self.sprite_data_blank(),
        }
    }
}

impl BoardToken {
    fn sprite_data_player_x(&self) -> SpriteData {
        let mut sprite_data = SpriteData::new();


        sprite_data.add_line("XXX  XXX");
        sprite_data.add_line("  XXXX  ");
        sprite_data.add_line("   XX   ");
        sprite_data.add_line("  XXXX  ");
        sprite_data.add_line("XXX  XXX");

        sprite_data
    }

    fn sprite_data_player_o(&self) -> SpriteData {
        let mut sprite_data = SpriteData::new();

        sprite_data.add_line("  OOOO  ");
        sprite_data.add_line("OOO  OOO");
        sprite_data.add_line("OO    OO");
        sprite_data.add_line("OOO  OOO");
        sprite_data.add_line("  OOOO  ");

        sprite_data
    }

    fn sprite_data_blank(&self) -> SpriteData {
    let mut sprite_data = SpriteData::new();

        sprite_data.add_line("        ");
        sprite_data.add_line("        ");
        sprite_data.add_line("        ");
        sprite_data.add_line("        ");
        sprite_data.add_line("        ");

        sprite_data
    }
}
