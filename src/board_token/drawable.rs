use board_token::BoardToken;
use sprite::sprite_data::SpriteData;
use sprite::sprite_data_traits::Drawable;

impl Drawable for BoardToken {
    fn to_sprite_data(&self) -> SpriteData {
        match *self {
            BoardToken::Player1 => self.sprite_data_player1(),
            BoardToken::Player2 => self.sprite_data_player2(),
            BoardToken::Blank => self.sprite_data_blank(),
        }
    }
}

impl BoardToken {
    fn sprite_data_player1(&self) -> SpriteData {
        let mut sprite_data = SpriteData::new();

        sprite_data.add_line("XXX  XXX");
        sprite_data.add_line("  XXXX  ");
        sprite_data.add_line("   XX   ");
        sprite_data.add_line("  XXXX  ");
        sprite_data.add_line("XXX  XXX");

        sprite_data
    }

    fn sprite_data_player2(&self) -> SpriteData {
        let mut sprite_data = SpriteData::new();
        sprite_data.add_line("  OOOO  ");
        sprite_data.add_line("OOO  OOO");
        sprite_data.add_line("OO    OO");
        sprite_data.add_line("OOO  OOO");
        sprite_data.add_line("  OOOO  ");
        sprite_data
    }

    fn sprite_data_blank(&self) -> SpriteData {
        SpriteData::new()
    }
}
