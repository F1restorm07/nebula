use embedded_io::Read;
use crate::event::{ parse_event, Event };
use crate::error::NebulaError;

pub struct EventIterator<R: Read> {
    source: R,
}

impl<R: Read> EventIterator<R> {
    pub fn new(source: impl Into<R>) -> Self { Self { source: source.into() } }
}

impl<R: Read> Iterator for EventIterator<R> {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let source = &mut self.source;

        // this code feels stupid lmao XD
        let mut item = alloc::vec![0; 2]; // ??: how large should the initial capacity be
        source.read(&mut item).unwrap();
        loop {
            match parse_event(&item[..]) {
                Ok(ev) => return Some(ev),
                Err(NebulaError::NeedMoreData(hint)) => {
                    item.extend((0..hint).map(|_| 0u8));
                    source.read(&mut item).unwrap();
                    continue;
                },
                _ => return None,
            };
        }
    }
}
