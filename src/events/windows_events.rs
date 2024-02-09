// use core::time;
use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    /*MOUSE_EVENT_FLAGS, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,*/ MOUSEEVENTF_MOVE,
    MOUSEINPUT, VIRTUAL_KEY, VK_RETURN,
};

const CBSIZE: i32 = size_of::<INPUT>() as i32;

pub fn win_write_char(char: char) {
    let char_repr = match char {
        c if c.is_alphanumeric() => c.to_string(),
        '\n' => "EOL".to_owned(),
        '\t' => "TAB".to_owned(),
        '\r' => "CAR".to_owned(),
        ' ' => "SPC".to_owned(),
        _ => "NON".to_owned(),
    };
    println!("sending char: {}, {}", char as u32, char_repr);

    match char {
        '\n' => send_vk(VK_RETURN),
        _ => send_unicode(char),
    }
}

pub fn win_move_mouse(dx: i32, dy: i32) {
    let inputs: [INPUT; 1] = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: dx,
                dy: dy,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];
    send_input(&inputs);
}

pub fn win_lmb_down() {
    let inputs: [INPUT; 1] = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTDOWN,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];
    send_input(&inputs);
}

pub fn win_lmb_up() {
    let inputs: [INPUT; 1] = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];
    send_input(&inputs);
}

fn send_unicode(char: char) {
    let inputs: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: char as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: char as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    send_input(&inputs);
}

fn send_vk(vk: VIRTUAL_KEY) {
    let inputs: [INPUT; 2] = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    send_input(&inputs);
}

fn send_input(inputs: &[INPUT]) {
    unsafe {
        SendInput(inputs, CBSIZE);
        // let error = GetLastError();
        // if error != 0 {
        //     println!("error while executing event {}", error)
        // }
    }
}
