extern crate flate2;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate snap;
extern crate protobuf;

pub mod compression;
pub mod fs;
mod orc_proto;
pub mod reader;
pub mod ty;
pub mod version;

use OrcErr::*;
use std::error::Error;
use std::fmt;
use std::io;
use std::i32;

pub enum OrcErr {
  IoError(io::Error),
  MalformedOrcFormat(String),
  InvalidCompressionFormat(String),
  UnsupportedCompressionCodec(String)
}

impl Error for OrcErr {
    fn description(&self) -> &str {
      match *self {
        IoError(ref ioe) => ioe.description(),
        MalformedOrcFormat(ref m) => &m,
        InvalidCompressionFormat(ref m) => &m,
        UnsupportedCompressionCodec(ref m) => &m,
      }
    }

    fn cause(&self) -> Option<&Error> {
      match *self {
        IoError(ref ioe) => Some(ioe),
        _ => None
      }
    }
}

impl fmt::Display for OrcErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Debug for OrcErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

pub type OrcResult<T> = Result<T, OrcErr>;

impl From<io::Error> for OrcErr {
    fn from(ioe: io::Error) -> Self {
      OrcErr::IoError(ioe)
    }
}