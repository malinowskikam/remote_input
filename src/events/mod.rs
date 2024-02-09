use self::windows_events::{win_lmb_down, win_lmb_up, win_move_mouse, win_write_char};

mod windows_events;

pub fn write_char(char: char) {
    win_write_char(char);
}

pub fn write_string(string: String) {
    string.chars().for_each(|c| write_char(c))
}

pub fn move_mouse(dx: i32, dy: i32) {
    win_move_mouse(dx, dy)
}

pub fn lmb_down() {
    win_lmb_down()
}

pub fn lmb_up() {
    win_lmb_up()
}
