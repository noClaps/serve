use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    TinyHttpError(Box<dyn error::Error + Send + Sync>),
    FileTypeError(file_type::Error),
    UnitError,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::IoError(err) => err.to_string(),
                Self::TinyHttpError(err) => err.to_string(),
                Self::FileTypeError(err) => err.to_string(),
                Self::UnitError => "Unknown error".to_string(),
            },
        )
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<Box<dyn error::Error + Send + Sync>> for Error {
    fn from(value: Box<dyn error::Error + Send + Sync>) -> Self {
        Self::TinyHttpError(value)
    }
}

impl From<file_type::Error> for Error {
    fn from(value: file_type::Error) -> Self {
        Self::FileTypeError(value)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::UnitError
    }
}
