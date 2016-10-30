use sprite::sprite_data::SpriteData;

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
    sprite_data.add_line("*        *");
    sprite_data.add_line("**********");

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

pub fn buttom_bar() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("Q: Quit   R: Restart Game / Rematch");

    sprite_data
}
