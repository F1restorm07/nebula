use alloc::string::String;
use crate::csi;
use numtoa::NumToA;

pub fn to_alt_buffer() -> String {
    csi!("?1049h").into()
}
pub fn to_main_buffer() -> String {
    csi!("?1049l").into()
}

pub fn save() -> String {
    csi!("?47h").into()
}
pub fn restore() -> String {
    csi!("?47l").into()
}

pub fn erase_screen_to_end() -> String {
    csi!("0J").into()
}
pub fn erase_screen_to_start() -> String {
    csi!("1J").into()
}
pub fn erase_screen() -> String {
    csi!("2J").into()
}
pub fn erase_line_to_end() -> String {
    csi!("0K").into()
}
pub fn erase_line_to_start() -> String {
    csi!("1K").into()
}
pub fn erase_line() -> String {
    csi!("2K").into()
}

pub fn scroll_up(n: u16) -> String {
    let mut nb = [0; 5];
    ["\x1b[", n.numtoa_str(10, &mut nb), "S"].concat()
}
pub fn scroll_down(n: u16) -> String {
    let mut nb = [0; 5];
    ["\x1b[", n.numtoa_str(10, &mut nb), "T"].concat()
}
