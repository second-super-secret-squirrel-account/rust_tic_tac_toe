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

pub fn background(height: i32, width: i32) -> SpriteData {
    let mut sprite_data = SpriteData::new();

    let mut first_and_last_line = "+".to_string();
    let mut middle_line = "|".to_string();
    for _ in 2..width {
        first_and_last_line = first_and_last_line + "-";
        middle_line = middle_line + " "
    }
    first_and_last_line = first_and_last_line + "+";
    middle_line = middle_line + "|";

    sprite_data.add_line(first_and_last_line.clone());

    for _ in 2..height {
        sprite_data.add_line(middle_line.clone());
    }

    sprite_data.add_line(first_and_last_line);

    sprite_data
}



pub fn title() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("___________ .___  _________    ___________    _____    _________    ___________ ________    ___________");
    sprite_data.add_line("\\__    ___/ |   | \\_   ___ \\   \\__    ___/   /  _  \\   \\_   ___ \\   \\__    ___/ \\_____  \\   \\_   _____/");
    sprite_data.add_line("  |    |    |   | /    \\  \\/     |    |     /  /_\\  \\  /    \\  \\/     |    |     /   |   \\   |    __)_ ");
    sprite_data.add_line("  |    |    |   | \\     \\____    |    |    /    |    \\ \\     \\____    |    |    /    |    \\  |        \\");
    sprite_data.add_line("  |____|    |___|  \\______  /    |____|    \\____|__  /  \\______  /    |____|    \\_______  / /_______  /");
    sprite_data.add_line("                          \\/                       \\/          \\/                       \\/          \\/ ");

    sprite_data
}

pub fn bottom_menu() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("Q: Quit   R: Rematch");

    sprite_data
}
