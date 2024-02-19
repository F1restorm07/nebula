use crate::event::{ parse_event, Event };
use crate::error::NebulaError;

pub struct EventIterator<'e> {
    buffer: &'e [u8],
    offset: usize
}

impl<'e> EventIterator<'e> {
    pub fn new(buffer: &'e [u8]) -> Self { Self { buffer, offset: 0 } }
}

impl Iterator for EventIterator<'_> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.buffer.len() { return None; }

        // this code feels stupid lmao XD
        let mut inc = 1;
        let mut item = &self.buffer[self.offset..(self.offset + inc)];
        loop {
            match parse_event(item) {
                Ok(ev) => {
                    self.offset+=inc;
                    return Some(ev);
                }
                Err(NebulaError::NeedMoreData) => {
                    // add another byte to the item and try again
                    item = &self.buffer[self.offset..=(self.offset + inc)];
                    inc+=1;
                    continue;
                },
                _ => return None,
            };
        }
    }
}
