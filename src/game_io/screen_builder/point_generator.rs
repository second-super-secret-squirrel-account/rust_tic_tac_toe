use game_io::game_screen::sprite::point::Point;

pub fn center(item_height: i32,
              item_width: i32,
              screen_height: i32,
              screen_width: i32)
              -> Point {
    let x = center_side(item_height, screen_height);
    let y = center_side(item_width, screen_width);
    return Point::new(x, y);
}


pub fn center_side(item_side: i32, screen_side: i32) -> i32 {
    (screen_side / 2) - (item_side / 2)
}
