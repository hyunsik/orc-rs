extern crate protobuf;

pub mod fs;
mod orc_proto;
pub mod ty;

use std::io;

pub enum OrcError {
  IoError(io::Error)
}

pub type OrcResult<T> = Result<T, OrcError>;

impl From<io::Error> for OrcError {
    fn from(ioe: io::Error) -> Self {
      OrcError::IoError(ioe)
    }
}
