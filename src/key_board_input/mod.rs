use std::collections::HashMap;
use ncurses_wrapper;

pub const KEY_UP: i32 = ncurses_wrapper::KEY_UP;
pub const KEY_DOWN: i32 = ncurses_wrapper::KEY_DOWN;
pub const KEY_RIGHT: i32 = ncurses_wrapper::KEY_RIGHT;
pub const KEY_LEFT: i32 = ncurses_wrapper::KEY_LEFT;

pub const KEY_J: i32 = 106;
pub const KEY_K: i32 = 107;
pub const KEY_L: i32 = 108;
pub const KEY_SEMI_COLON: i32 = 59;

pub const KEY_W: i32 = 119;
pub const KEY_A: i32 = 97;
pub const KEY_S: i32 = 115;
pub const KEY_D: i32 = 100;

pub const KEY_Q: i32 = 113;
pub const KEY_R: i32 = 114;

pub const KEY_ENTER: i32 = 10;
pub const KEY_SPACEBAR: i32 = 32;



pub struct KeyBoardInput {
    key_commands: HashMap<i32, fn()>,

}


impl KeyBoardInput {
    pub fn new() -> KeyBoardInput {
        let map = HashMap::<i32, fn()>::new();
        KeyBoardInput {
            key_commands: map,
        }
    }

    pub fn on_key(&mut self, key: i32, command: fn()) {
        self.key_commands.insert(key, command);
    }

    pub fn run_command_for_key(&self, key: i32) {

        if let Some(command) = self.key_commands.get(&key) {
            command;
        }
    }
}
