use alloc::string::String;
use crate::csi;
use numtoa::NumToA;

// TODO: remove all allocations

pub fn black_fg(text: impl AsRef<str>) -> String {
    [csi!("30"), text.as_ref(), csi!("39m")].concat()
}
pub fn red_fg(text: impl AsRef<str>) -> String {
    [csi!("31m"), text.as_ref(), csi!("39m")].concat()
}
pub fn green_fg(text: impl AsRef<str>) -> String {
    [csi!("32m"), text.as_ref(), csi!("39m")].concat()
}
pub fn yellow_fg(text: impl AsRef<str>) -> String {
    [csi!("33m"), text.as_ref(), csi!("39m")].concat()
}
pub fn blue_fg(text: impl AsRef<str>) -> String {
    [csi!("34m"), text.as_ref(), csi!("39m")].concat()
}
pub fn magenta_fg(text: impl AsRef<str>) -> String {
    [csi!("35m"), text.as_ref(), csi!("39m")].concat()
}
pub fn cyan_fg(text: impl AsRef<str>) -> String {
    [csi!("36m"), text.as_ref(), csi!("39m")].concat()
}
pub fn white_fg(text: impl AsRef<str>) -> String {
    [csi!("37m"), text.as_ref(), csi!("39m")].concat()
}

pub fn bright_black_fg(text: impl AsRef<str>) -> String {
    [csi!("90"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_red_fg(text: impl AsRef<str>) -> String {
    [csi!("91m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_green_fg(text: impl AsRef<str>) -> String {
    [csi!("92m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_yellow_fg(text: impl AsRef<str>) -> String {
    [csi!("93m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_blue_fg(text: impl AsRef<str>) -> String {
    [csi!("94m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_magenta_fg(text: impl AsRef<str>) -> String {
    [csi!("95m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_cyan_fg(text: impl AsRef<str>) -> String {
    [csi!("96m"), text.as_ref(), csi!("39m")].concat()
}
pub fn bright_white_fg(text: impl AsRef<str>) -> String {
    [csi!("97m"), text.as_ref(), csi!("39m")].concat()
}

pub fn black_bg(text: impl AsRef<str>) -> String {
    [csi!("40"), text.as_ref(), csi!("49m")].concat()
}
pub fn red_bg(text: impl AsRef<str>) -> String {
    [csi!("41m"), text.as_ref(), csi!("49m")].concat()
}
pub fn green_bg(text: impl AsRef<str>) -> String {
    [csi!("42m"), text.as_ref(), csi!("49m")].concat()
}
pub fn yellow_bg(text: impl AsRef<str>) -> String {
    [csi!("43m"), text.as_ref(), csi!("49m")].concat()
}
pub fn blue_bg(text: impl AsRef<str>) -> String {
    [csi!("44m"), text.as_ref(), csi!("49m")].concat()
}
pub fn magenta_bg(text: impl AsRef<str>) -> String {
    [csi!("45m"), text.as_ref(), csi!("49m")].concat()
}
pub fn cyan_bg(text: impl AsRef<str>) -> String {
    [csi!("46m"), text.as_ref(), csi!("49m")].concat()
}
pub fn white_bg(text: impl AsRef<str>) -> String {
    [csi!("47m"), text.as_ref(), csi!("49m")].concat()
}

pub fn bright_black_bg(text: impl AsRef<str>) -> String {
    [csi!("100"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_red_bg(text: impl AsRef<str>) -> String {
    [csi!("101m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_green_bg(text: impl AsRef<str>) -> String {
    [csi!("102m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_yellow_bg(text: impl AsRef<str>) -> String {
    [csi!("103m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_blue_bg(text: impl AsRef<str>) -> String {
    [csi!("104m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_magenta_bg(text: impl AsRef<str>) -> String {
    [csi!("105m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_cyan_bg(text: impl AsRef<str>) -> String {
    [csi!("106m"), text.as_ref(), csi!("49m")].concat()
}
pub fn bright_white_bg(text: impl AsRef<str>) -> String {
    [csi!("107m"), text.as_ref(), csi!("49m")].concat()
}

pub fn xterm_fg(text: impl AsRef<str>, r: u8, g: u8, b: u8) -> String {
    let mut idx_b = [0; 3];
    let idx = (36*r)+(6*g)+b+16;
    [csi!("38;5;"), idx.numtoa_str(10, &mut idx_b), "m", text.as_ref(), csi!("0m")].concat()
}
pub fn xterm_bg(text: impl AsRef<str>, r: u8, g: u8, b: u8) -> String {
    let mut idx_b = [0; 3];
    let idx = (36*r)+(6*g)+b+16;
    [csi!("48;5;"), idx.numtoa_str(10, &mut idx_b), "m", text.as_ref(), csi!("0m")].concat()
}
pub fn gray_fg(text: impl AsRef<str>, shade: u8) -> String {
    let mut idx_b = [0; 3];
    let shade = 232+shade;
    [csi!("38;5;"), shade.numtoa_str(10, &mut idx_b), "m", text.as_ref(), csi!("0m")].concat()
}
pub fn gray_bg(text: impl AsRef<str>, shade: u8) -> String {
    let mut idx_b = [0; 3];
    let shade = 232+shade;
    [csi!("48;5;"), shade.numtoa_str(10, &mut idx_b), "m", text.as_ref(), csi!("0m")].concat()
}

pub fn truecolor_fg(text: impl AsRef<str>, r: u8, g: u8, b: u8) -> String {
    let (mut rb, mut gb, mut bb) = ([0; 3], [0; 3], [0; 3]);
    let (r, g, b) = (r.numtoa_str(10, &mut rb), g.numtoa_str(10, &mut gb), b.numtoa_str(10, &mut bb));
    [csi!("38;2;"), r, ";", g, ";", b, "m", text.as_ref(), csi!("0m")].concat()
}
pub fn truecolor_bg(text: impl AsRef<str>, r: u8, g: u8, b: u8) -> String {
    let (mut rb, mut gb, mut bb) = ([0; 3], [0; 3], [0; 3]);
    let (r, g, b) = (r.numtoa_str(10, &mut rb), g.numtoa_str(10, &mut gb), b.numtoa_str(10, &mut bb));
    [csi!("48;2;"), r, ";", g, ";", b, "m", text.as_ref(), csi!("0m")].concat()
}
