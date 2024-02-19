// TODO: properly handle u16 and other multi-byte character sequences
// |--- parse u16 from &str
use alloc::boxed::Box;
use crate::error::{ Result, NebulaError };

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    FocusGained,
    FocusLost,
}

#[derive(Debug)]
pub enum KeyEvent {
    Up, Down, Left, Right,
    Backspace, Delete, Insert, End, Home,
    Tab, BackTab, // BackTab --> shift + tab
    PageUp, PageDown,
    Null, Esc, Enter,
    Func(u8), Char(char), Modified(ModifierKey, Box<Self>),
}
#[derive(Debug)]
pub enum ModifierKey { Alt, Ctrl, Super, Func, Shift } // TODO: support multiple pressed at once
                                                       // may use array in KeyEvent::Modifier

#[derive(Debug)]
pub enum MouseEvent {
    Press(MouseButton, u16, u16),
    Release(u16, u16),
    Drag(MouseButton, u16, u16),
    Position(u16, u16),
    Moved,
    ScrollUp, ScrollDown, ScrollLeft, ScrollRight,
}
#[derive(Debug)]
pub enum MouseButton { Left, Middle, Right }

pub fn parse_event(item: &[u8]) -> Result<Event> {
    match item[0] {
        b'\x1b' => {
            if item.len() == 1 { return Ok(Event::Key(KeyEvent::Esc)); }

            match item[1] {
                b'O' => {
                    if item.len() == 2 { return Err(NebulaError::NeedMoreData); }

                    match item[2] {
                        b'A' => Ok(Event::Key(KeyEvent::Up)),
                        b'B' => Ok(Event::Key(KeyEvent::Down)),
                        b'C' => Ok(Event::Key(KeyEvent::Right)),
                        b'D' => Ok(Event::Key(KeyEvent::Left)),
                        b'F' => Ok(Event::Key(KeyEvent::End)),
                        b'H' => Ok(Event::Key(KeyEvent::Home)),
                        val @ b'P'..=b'S' => Ok(Event::Key(KeyEvent::Func(1 + val - b'P'))),
                        _ => Err(NebulaError::UnknownSequence),
                    }
                },
                b'[' => parse_csi(item),
                b'\x1b' => Ok(Event::Key(KeyEvent::Esc)),
                _ => parse_event(&item[1..]).map(|ev| {
                    if let Event::Key(kev) = ev {
                        Event::Key(KeyEvent::Modified(ModifierKey::Alt, Box::new(kev)))
                    } else { ev }
                })
            }
        },
        b'\r' | b'\n' => Ok(Event::Key(KeyEvent::Enter)),
        b'\t' => Ok(Event::Key(KeyEvent::Tab)),
        b'\x7f' => Ok(Event::Key(KeyEvent::Backspace)),
        c @ b'\x01'..=b'\x1a' => Ok(Event::Key(KeyEvent::Modified(ModifierKey::Ctrl, Box::new(KeyEvent::Char((c - 0x1 + b'a') as char))))),
        c @ b'\x1c'..=b'\x1f' => Ok(Event::Key(KeyEvent::Modified(ModifierKey::Ctrl, Box::new(KeyEvent::Char((c - 0x1 + b'4') as char))))),
        b'\0' => Ok(Event::Key(KeyEvent::Modified(ModifierKey::Ctrl, Box::new(KeyEvent::Char(' '))))),
        c => Ok(Event::Key(KeyEvent::Char(c as char))),
    }
}

fn parse_csi(item: &[u8]) -> Result<Event> {
    assert!(item.starts_with(&[b'\x1b', b'[']));

    if item.len() == 2 { return Err(NebulaError::NeedMoreData); }

    match item[2] {
        b'[' => { unimplemented!(); } // ??: not sure what to do here
        b'A' => Ok(Event::Key(KeyEvent::Up)),
        b'B' => Ok(Event::Key(KeyEvent::Down)),
        b'C' => Ok(Event::Key(KeyEvent::Right)),
        b'D' => Ok(Event::Key(KeyEvent::Left)),
        b'F' => Ok(Event::Key(KeyEvent::End)),
        b'H' => Ok(Event::Key(KeyEvent::Home)),
        b'Z' => Ok(Event::Key(KeyEvent::BackTab)),
        // TODO: (P, Q, S) --> read up on kitty term keyboard protocol
        b'P' => Ok(Event::Key(KeyEvent::Func(1))),
        b'Q' => Ok(Event::Key(KeyEvent::Func(2))),
        b'S' => Ok(Event::Key(KeyEvent::Func(4))),
        b'I' => Ok(Event::FocusGained),
        b'O' => Ok(Event::FocusLost),
        b'M' => parse_x10_mouse_event(item),
        b'<' => parse_xterm_mouse_event(item),
        b';' => parse_csi_modifier(item),
        b'?' => match item[item.len() - 1] {
            _ => Err(NebulaError::NeedMoreData), // TODO: implement attribute and device enhancements (after everything else)
        },
        b'0'..=b'9' => {
            // TODO: implement bracketed paste later (after everything else)
            if item.len() == 3 { Err(NebulaError::NeedMoreData) }
            else {
                let last_byte = item[item.len() - 1];
                if !(64..=126).contains(&last_byte) { Err(NebulaError::NeedMoreData) }
                else {
                    match last_byte {
                        b'M' => parse_rxvt_mouse_event(item),
                        b'~' => Ok(Event::Key(parse_csi_special_keycode(item))),
                        b'u' => Ok(Event::Key(parse_unicode_keycode(item))),
                        b'R' => Ok(Event::Mouse(parse_cursor_position(item))),
                        _ => parse_csi_modifier(item),
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
}
 fn parse_x10_mouse_event(item: &[u8]) -> Result<Event> {
    // CSI M <mouse button> <x> <y>
    assert!(item.starts_with(b"\x1b[m"));
    if item.len() < 6 { return Err(NebulaError::NeedMoreData); }
    // TODO: parse the mouse button byte
    let button = item[3].checked_sub(32).unwrap();
    let x = u16::from(item[4].saturating_sub(32)) - 1;
    let y = u16::from(item[5].saturating_sub(32)) - 1;

    Ok(Event::Mouse(parse_mouse_button_byte(button, x, y)))
}
fn parse_xterm_mouse_event(item: &[u8]) -> Result<Event> {
    // CSI < <mouse button> ; <x> ; <y> (;) (M/m)
    assert!(item.starts_with(b"\x1b[<"));
    if !item.ends_with(&[b'm']) && !item.ends_with(&[b'M']) { panic!(); }

    let mut params = item[3..item.len() - 1].split(|&b| b == b';');
    let button = params.next().unwrap()[0];
    let (x, y) = (parse_to::<u16>(params.next().unwrap()) - 1, parse_to::<u16>(params.next().unwrap()) - 1);

    // i don't care about parsing the last byte (M/m) right now (m -> release, M -> press)
    Ok(Event::Mouse(parse_mouse_button_byte(button, x, y)))
}
fn parse_rxvt_mouse_event(item: &[u8]) -> Result<Event> {
    // CSI <mouse button> ; <x> ; <y> M

    let mut params = item[3..item.len() - 1].split(|&b| b == b';');
    let button = params.next().unwrap()[0];
    let (x, y) = (parse_to::<u16>(params.next().unwrap()) - 1, parse_to::<u16>(params.next().unwrap()) - 1);

    Ok(Event::Mouse(parse_mouse_button_byte(button, x, y)))
}
fn parse_csi_special_keycode(item: &[u8]) -> KeyEvent {
    // TODO: parse modifiers
    let mut params = item[2..item.len() - 1].split(|&b| b == b';');
    let first = parse_to::<u8>(params.next().unwrap());

    match first {
        1 | 7 => KeyEvent::Home,
        2 => KeyEvent::Insert,
        3 => KeyEvent::Delete,
        4 | 8 => KeyEvent::End,
        5 => KeyEvent::PageUp,
        6 => KeyEvent::PageDown,
        b @ 11..=15 => KeyEvent::Func(b - 10),
        b @ 17..=21 => KeyEvent::Func(b - 11),
        b @ 23..=26 => KeyEvent::Func(b - 12),
        b @ 28..=29 => KeyEvent::Func(b - 15),
        b @ 31..=34 => KeyEvent::Func(b - 17),
        _ => panic!(), // TODO: Err(NebulaError::UnsupportedSequence)
    }
}
fn parse_unicode_keycode(item: &[u8]) -> KeyEvent {
    // CSI codepoint ; modifiers u
    // see https://www.leonerd.org.uk/hacks/fixterms/ for more info
    //
    // the kitty keyboard protocol extends the CSI u sequence as listed below (progessively opted in to)
    // CSI <unicode_keycode>:<alt_keycodes> ; <modifiers>:<event_type> ; <text_codepoints> u
    // see https://sw.kovidgoyal.net/kitty/keyboard-protocol/ for more info

    // TODO: check for optional parameters, as only the unicode_keycode is required

    let mut params = item[2..item.len() - 1].split(|&b| b == b';');

    let mut keycodes = params.next().unwrap().split(|&b| b == b':');
    // the unicode keycode is always unshifted (A -> a)
    let unicode_keycode = parse_utf8_char(keycodes.next().unwrap());
    // let shifted_keycode = keycodes.next().unwrap();
    // let base_layout_keycode = keycodes.next().unwrap();

    // let modifiers = params.next().unwrap().split(|&b| b == b':');

    match unicode_keycode {
        '\x1b' => KeyEvent::Esc,
        '\r' | '\n' => KeyEvent::Enter,
        '\t' => KeyEvent::Tab, // TODO: handle BackTab with Shift modifier
        '\x7f' => KeyEvent::Backspace,
        _ => KeyEvent::Char(unicode_keycode),
    }
}
fn parse_cursor_position(item: &[u8]) -> MouseEvent {
    // CSI <row> ; <col> R

    let mut params = item[2..item.len() - 1].split(|&b| b == b';');
    let (row, col) = (u16::from(params.next().unwrap()[0]) - 1, u16::from(params.next().unwrap()[0]) - 1);

    MouseEvent::Position(row, col)
}
// TODO: figure out what modifier keycodes are and parse them
fn parse_csi_modifier(_item: &[u8]) -> Result<Event> {
    Ok(Event::Key(KeyEvent::Char('a')))
}
// the x and y parameters are only for constructing the mouse event at the end
fn parse_mouse_button_byte(button: u8, x: u16, y: u16) -> MouseEvent {
    // bits (lo to hi)
    // 1-2: button number
    // 3: shift?
    // 4: alt?
    // 5: ctrl?
    // 6: dragging?
    // 7-8: button number

    // shifting the hi bits down gets us to some nice low numbers
    let button_number = (button & 0b0000_0011) | ((button & 0b1100_0000)>>4);
    let dragging = button & 0b0010_0000 == 0b0010_0000;

    // TODO: parse the modifiers
    match (button_number, dragging) {
        (0, false) => MouseEvent::Press(MouseButton::Left, x, y),
        (1, false) => MouseEvent::Press(MouseButton::Middle, x, y),
        (2, false) => MouseEvent::Press(MouseButton::Right, x, y),
        (3, false) => MouseEvent::Release(x, y),
        (0, true) => MouseEvent::Drag(MouseButton::Left, x, y),
        (1, true) => MouseEvent::Drag(MouseButton::Middle, x, y),
        (2, true) => MouseEvent::Drag(MouseButton::Right, x, y),
        (3, true) | (4, true) | (5, true) => MouseEvent::Moved,
        (4, false) => MouseEvent::ScrollUp,
        (5, false) => MouseEvent::ScrollDown,
        (6, false) => MouseEvent::ScrollLeft,
        (7, false) => MouseEvent::ScrollRight,
        _ => unimplemented!(),
    }
}
fn parse_to<T>(item: &[u8]) -> T
where
    T: core::str::FromStr,
    T::Err: core::fmt::Debug,
{
    core::str::from_utf8(item).unwrap().parse::<T>().unwrap()
}
fn parse_utf8_char(item: &[u8]) -> char {
    match core::str::from_utf8(item) {
        Ok(s) => s.chars().next().unwrap(),
        Err(_) => unimplemented!(), // TODO: parse non-utf8 chars + chars that need more data
    }
}
