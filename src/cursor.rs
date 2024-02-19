use alloc::string::String;
use crate::csi;
use numtoa::NumToA;

pub fn goto_home() -> String {
    csi!("H").into()
}
/// position the cursor at a specific coordinate (one-based)
pub fn goto(line: u16, col: u16) -> String {
    let (mut lb, mut cb) = ([0; 4], [0; 4]);
    let (line, col) = (line.numtoa_str(10, &mut lb), col.numtoa_str(10, &mut cb));
    ["\x1b[", line, ";", col, "H"].concat()
}
pub fn up(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "A"].concat()
}
pub fn down(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "B"].concat()
}
pub fn right(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "C"].concat()
}
pub fn left(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "D"].concat()
}
pub fn down_start(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "E"].concat()
}
pub fn up_start(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "F"].concat()
}
pub fn col(n: u16) -> String {
    let mut nb = [0; 4];
    ["\x1b[", n.numtoa_str(10, &mut nb), "G"].concat()
}

pub fn hide() -> String {
    csi!("?25l").into()
}
pub fn show() -> String {
    csi!("?25h").into()
}

pub fn save() -> String {
    csi!("s").into()
}
pub fn restore() -> String {
    csi!("u").into()
}
