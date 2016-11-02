use game_io::game_screen::sprite::sprite_data::SpriteData;

pub fn board() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("+----------+----------+----------+");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("+----------+----------+----------+");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("+----------+----------+----------+");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("|          |          |          |");
    sprite_data.add_line("+----------+----------+----------+");

    sprite_data
}

pub fn board_cursor() -> SpriteData {
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


pub fn bottom_menu() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("q: Quit   r: Rematch");

    sprite_data
}
