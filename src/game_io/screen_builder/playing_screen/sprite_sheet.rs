use game_io::game_screen::sprite::sprite_data::SpriteData;
use game_board::board_token::BoardToken;

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

pub fn winner(board_token: BoardToken) -> SpriteData {
    let mut sprite_data = SpriteData::new();
    match board_token {
        BoardToken::Blank => sprite_data.add_line("Tied"),
        BoardToken::PlayerX => sprite_data.add_line("X is the Winner"),
        BoardToken::PlayerO => sprite_data.add_line("O is the Winner"),
    }

    sprite_data
}

pub fn bottom_menu() -> SpriteData {
    let mut sprite_data = SpriteData::new();

    sprite_data.add_line("q: Quit   r: Rematch");

    sprite_data
}
