use std::fmt;
use std::io::Error as IOError;
use postcard::Error as PostCardError;

#[derive(Debug, PartialEq)]
pub enum ZipErrOrigin {
    IO,
    PostCard
}

#[derive(Debug, PartialEq)]
pub struct ZipError {
    _err_type: ZipErrOrigin,
    msg: String
}

impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<IOError> for ZipError {
    fn from(origin: IOError) -> Self {
        Self { _err_type: ZipErrOrigin::IO, msg: origin.to_string() }
    }
}

impl From<PostCardError> for ZipError {
    fn from(origin: PostCardError) -> Self {
        Self { _err_type: ZipErrOrigin::PostCard, msg: origin.to_string() }
    }
}

