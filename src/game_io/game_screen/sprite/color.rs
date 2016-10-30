use game_io::ncurses_wrapper::color_pairs::*;

#[derive(Debug, Eq, PartialEq, Clone)]
 pub enum Color {
     WhiteOnBlue,
     RedOnBlue,
     GreenOnBlue,
     YellowOnBlue,
     RedOnYellow,
     GreenOnYellow,
     BlueOnYellow,
}

impl Color {
    pub fn value(&self) -> i16 {
        match *self {
            Color::WhiteOnBlue => WHITE_ON_BLUE,
            Color::RedOnBlue => RED_ON_BLUE,
            Color::GreenOnBlue => GREEN_ON_BLUE,
            Color::YellowOnBlue => YELLOW_ON_BLUE,
            Color::RedOnYellow => RED_ON_YELLOW,
            Color::GreenOnYellow => GREEN_ON_YELLOW,
            Color::BlueOnYellow => BLUE_ON_YELLOW,
        }
    }
}
