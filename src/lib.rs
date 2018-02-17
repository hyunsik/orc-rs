extern crate env_logger;
#[macro_use] extern crate log;
extern crate protobuf;

pub mod fs;
mod orc_proto;
pub mod reader;
pub mod ty;

use std::io;

pub enum OrcErr {
  IoError(io::Error),
  MalformedFormat(String)
}

pub type OrcResult<T> = Result<T, OrcErr>;

impl From<io::Error> for OrcErr {
    fn from(ioe: io::Error) -> Self {
      OrcErr::IoError(ioe)
    }
}
