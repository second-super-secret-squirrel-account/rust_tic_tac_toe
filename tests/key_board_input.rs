extern crate tic_tac_toe;

use tic_tac_toe::key_board_input;
use tic_tac_toe::key_board_input::KeyBoardInput;


struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }

    fn increase_count(&mut self) {
        self.count = self.count + 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

#[test]
fn running_a_command_that_was_set() {
    let mut key_commands = KeyBoardInput::new();
    let mut counter = Counter::new();
    key_commands.on_key(key_board_input::KEY_J, counter.increase_count);

    key_commands.run_command_for_key(key_board_input::KEY_J);

    assert_eq!(1, counter.get_count());
}
