use game_io::game_screen::sprite::sprite_data::SpriteData;

pub fn menu_item(index: usize) -> SpriteData {
    let mut sprite_data = SpriteData::new();
    if index == 0 {
        sprite_data.add_line("Human      vs     Human   ");
    } else if index == 1 {
        sprite_data.add_line("Human      vs     Computer");
    } else {
        sprite_data.add_line("Computer   vs     Human   ");
    }
    sprite_data
}

pub fn controls() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("+----------------------------------------------------+");
    sprite_data.add_line("|                    Controls                        |");
    sprite_data.add_line("|----------------------------------------------------|");
    sprite_data.add_line("|         Move Up:     w, k, or up-arrow             |");
    sprite_data.add_line("|         Move Down:   s, j, or down-arrow           |");
    sprite_data.add_line("|         Move Left:   a, h, or left-arrow           |");
    sprite_data.add_line("|         Move Right:  d, l, or right-arrow          |");
    sprite_data.add_line("|                                                    |");
    sprite_data.add_line("|         Select:      Return, or SpaceBar           |");
    sprite_data.add_line("|                                                    |");
    sprite_data.add_line("|         Rematch:     r                             |");
    sprite_data.add_line("|                                                    |");
    sprite_data.add_line("|         Quit:        q                             |");
    sprite_data.add_line("+----------------------------------------------------+");

    sprite_data
}

pub fn bottom_menu() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("q: Quit");

    sprite_data
}
