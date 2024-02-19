pub enum NebulaError {
    NeedMoreData,
    UnknownSequence,
    OsError(rustix::io::Errno),
}

pub type Result<T> = core::result::Result<T, NebulaError>;
