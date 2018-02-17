use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use super::OrcResult;

pub trait InputStream: Read + Seek {
  /// Get the total length of the file in bytes.
  fn len(&self) -> u64;

  fn read_at(&mut self, buf: &mut [u8], pos: u64) -> OrcResult<usize> {
    self.seek(SeekFrom::Start(pos))?;
    Ok(self.read(buf)?)
  }
  
  /// Get the number of bytes that should be read at once
  fn default_read_size(&self) -> u64;

  /// Get a name of the stream
  fn name(&self) -> &str;
}

pub trait OutputStream {
  /// Get the total length of bytes written.
  fn len(&self) -> u64;
  
  /// Get the number of bytes that should be written at once
  fn default_write_size(&self) -> u64;

  /// Write length bytes from the buffer into the stream
  fn write(&mut self, buf: &[u8]) -> OrcResult<()>;

  /// Get a name of the stream
  fn name(&self) -> &str;

  /// Close the stream and flush any pending data to the storage.
  fn close(&mut self);
}

pub struct FileInputStream {
  path: String,
  file: File
}

impl FileInputStream {
  pub fn open(path: &str) -> OrcResult<FileInputStream> {
    Ok(FileInputStream {
      path: path.to_owned(),
      file: File::open(path)?
    })
  }
}

const DEFAULT_FILE_READ_SIZE: u64 = 128 * 1024;

impl InputStream for FileInputStream {
  fn len(&self) -> u64 {
    self.file.metadata().expect("len()").len()
  }
  
  fn default_read_size(&self) -> u64 {
    DEFAULT_FILE_READ_SIZE
  }

  fn name(&self) -> &str {
    &self.path
  }
}

impl Read for FileInputStream {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.file.read(buf)
  }
}

impl Seek for FileInputStream {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    self.file.seek(pos)
  }
}