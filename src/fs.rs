use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::OrcResult;

pub trait InputStream {
  /// Get the total length of the file in bytes.
  fn len(&self) -> u64;
  
  /// Get the number of bytes that should be read at once
  fn default_read_size(&self) -> u64;

  /// Read length bytes from the file starting at offset into the buffer
  fn read(&mut self, buf: &mut [u8], offset: Option<u64>) -> OrcResult<usize>;

  /// Get a name of the stream
  fn name(&self) -> &str;

  /// Close the stream
  fn close(&mut self);
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

  fn read(&mut self, buf: &mut [u8], offset: Option<u64>) -> OrcResult<usize> {
    if let Some(pos) = offset {
      self.file.seek(SeekFrom::Start(pos))?;
    }

    Ok(self.file.read(buf)?)
  }

  fn name(&self) -> &str {
    &self.path
  }

  fn close(&mut self) {
    // nothing to do because a File are automatically closed when they go out of scope.
  }
}