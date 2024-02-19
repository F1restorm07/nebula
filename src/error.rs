#[derive(Debug)]
pub enum NebulaError {
    // the paramter is a hint to how many bytes are needed to produce the full event sequence
    NeedMoreData(usize),
    UnknownSequence,
    OsError(rustix::io::Errno),
}

pub type Result<T> = core::result::Result<T, NebulaError>;
