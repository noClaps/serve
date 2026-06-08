use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    TinyHttp(Box<dyn error::Error + Send + Sync>),
    FileType(file_type::Error),
    Unit,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::Io(err) => err.to_string(),
                Self::TinyHttp(err) => err.to_string(),
                Self::FileType(err) => err.to_string(),
                Self::Unit => "Unknown error".to_string(),
            },
        )
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Box<dyn error::Error + Send + Sync>> for Error {
    fn from(value: Box<dyn error::Error + Send + Sync>) -> Self {
        Self::TinyHttp(value)
    }
}

impl From<file_type::Error> for Error {
    fn from(value: file_type::Error) -> Self {
        Self::FileType(value)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::Unit
    }
}
