use alloc::string::String;
use crate::csi;

pub fn bold(text: impl AsRef<str>) -> String {
    [csi!("1m"), text.as_ref(), csi!("22m")].concat()
}
pub fn dim(text: impl AsRef<str>) -> String {
    [csi!("2m"), text.as_ref(), csi!("22m")].concat()
}
pub fn italic(text: impl AsRef<str>) -> String {
    [csi!("3m"), text.as_ref(), csi!("23m")].concat()
}
pub fn underline(text: impl AsRef<str>) -> String {
    [csi!("4m"), text.as_ref(), csi!("24m")].concat()
}
pub fn blink(text: impl AsRef<str>) -> String {
    [csi!("5m"), text.as_ref(), csi!("25m")].concat()
}
pub fn inverse(text: impl AsRef<str>) -> String {
    [csi!("7m"), text.as_ref(), csi!("27m")].concat()
}
pub fn hidden(text: impl AsRef<str>) -> String {
    [csi!("8m"), text.as_ref(), csi!("28m")].concat()
}
pub fn strikethrough(text: impl AsRef<str>) -> String {
    [csi!("9m"), text.as_ref(), csi!("29m")].concat()
}
