use std::fmt;
use sprite::sprite_data::SpriteData;
use sprite::sprite_data_traits::Drawable;
use grid::Grid;

impl <T: Clone + PartialEq> Drawable<T> for Grid<T> where T: fmt::Display {
    fn to_sprite_data(&self) -> SpriteData {

        let mut sprite_data = SpriteData::new();
        sprite_data.add_line(" ----- ----- ----- ");
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(self.create_row(0));
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(" ----- ----- ----- ");
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(self.create_row(1));
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(" ----- ----- ----- ");
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(self.create_row(2));
        sprite_data.add_line("|     |     |     |");
        sprite_data.add_line(" ----- ----- ----- ");

        sprite_data
    }
}

impl <T: Clone + PartialEq> Grid<T> where T: fmt::Display {
    fn create_row(&self, row: usize) -> String {
        let mut string_row = "|  ".to_string();
        string_row = string_row + &self.get(row, 0).to_string();
        string_row = string_row + "  |  ";
        string_row = string_row + &self.get(row, 1).to_string();
        string_row = string_row + "  |  ";
        string_row = string_row + &self.get(row, 2).to_string();
        string_row = string_row + "  |";
        string_row
    }
}
