use super::{OrcErr, OrcResult};
use compression::OrcCompressReader;
use fs::{FileReader, StreamReader};
use orc_proto::{self, CompressionKind, Footer, Metadata, PostScript};
use ty::Type;
use version::{self, Version};

use std::cmp;
use std::error::Error;
use std::str;
use protobuf::{CodedInputStream, Message};


static MAGIC_NUM: &'static [u8;3] = b"ORC";
static MAGIC_NUM_LEN: usize = 3;
static LEN_OF_POST_SCRIPT_LEN: usize = 1;
const DIRECTORY_SIZE_GUESS: usize = 16 * 1024;

pub trait RecordReader {
}

pub struct OrcRecordReader {  
}

impl RecordReader for OrcRecordReader {  
}

pub struct ReaderOptions {}

pub struct OrcReader {
  reader: Box<StreamReader>,
  //metadata: Metadata,
  footer: Footer,
  postscript: PostScript,
  file_len: u64,
  types: Type
}

impl OrcReader {
  pub fn records() -> OrcRecordReader {
    OrcRecordReader {}
  }

  pub fn compression_buffer_size(&self) -> u64 {
    self.postscript.get_compressionBlockSize()
  }

  pub fn compression_codec(&self) -> CompressionKind {
    self.postscript.get_compression()
  }

  pub fn row_num(&self) -> u64 {
    self.footer.get_numberOfRows()
  }

  pub fn contents_len(&self) -> u64 {
    self.footer.get_contentLength()
  }

  pub fn format_version(&self) -> &Version {
    version::find(self.postscript.get_version())
  }

  pub fn orc_writer_version(&self) -> u32 {
    self.postscript.get_writerVersion()
  }

  pub fn types(&self) -> Type {
    Type::from_proto(self.footer.get_types())
  }
}

struct OrcTail {
  postscript: PostScript,
  footer: Footer,
  file_len: u64,
  postscript_len: u64,
}

impl OrcReader {
  pub fn open(reader_: Box<StreamReader>) -> OrcResult<OrcReader> {
    let mut reader = reader_;
    let tail = {
      OrcReader::extract_tail(&mut *reader)?
    };

    Ok(OrcReader {
      reader: reader,
      postscript: tail.postscript,
      types: Type::from_proto(tail.footer.get_types()),
      footer: tail.footer,
      file_len: tail.file_len,      
    })
  }

  fn extract_tail(is: &mut StreamReader) -> OrcResult<OrcTail> {    
    let mut buf: [u8; DIRECTORY_SIZE_GUESS] = [0; DIRECTORY_SIZE_GUESS];
    let total_len = is.len();

    // read last bytes into buffer to get PostScript
    let read_size = cmp::min(total_len, DIRECTORY_SIZE_GUESS as u64);
    is.read_at(&mut buf, total_len - read_size)?;

    let ps_len = (buf[(read_size as usize - LEN_OF_POST_SCRIPT_LEN)] & 0xff) as usize; // Get a post script length
    OrcReader::ensure_footer(&buf, ps_len)?;

    let ps_offset = (read_size as usize) - 1 - ps_len;
    let ps = bytes_to_proto(&buf[ps_offset..ps_offset+ps_len], PostScript::new())?;
    let footer_len = ps.get_footerLength() as usize;
    let footer_offset = ps_offset - footer_len;
    let codec = ps.get_compression();
    let compressed_block_len = ps.get_compressionBlockSize();

    debug!("buf len: {}", buf.len());
    debug!("ps offset: {}", ps_offset);
    debug!("ps len: {}", ps_len);
    debug!("footer offset: {}", footer_offset);
    debug!("footer len: {}", footer_len);
    debug!("compression: {:?}", codec);
    debug!("compressed_block_len: {}", compressed_block_len);
    
    let footer = compressed_bytes_to_proto(&buf[footer_offset..footer_offset+footer_len],
      Footer::new(), codec)?;
    debug!("row num: {}", footer.get_numberOfRows());
    
    Ok(OrcTail {
      postscript: ps,
      footer: footer,
      file_len: total_len,
      postscript_len: ps_len as u64
    })
  }

  pub fn ensure_footer(buf: &[u8], ps_len: usize) -> OrcResult<()> {
    let magic_len = (MAGIC_NUM_LEN + LEN_OF_POST_SCRIPT_LEN) as usize; // length of magic number + post script length

    if ps_len < magic_len {
      return Err(OrcErr::MalformedOrcFormat(
        format!("Malformed ORC file. Invalid postscript length {}", ps_len)));
    }

    let magic_offset = buf.len() - magic_len;
    let magic_code = &buf[magic_offset..(magic_offset + MAGIC_NUM_LEN)];

    if MAGIC_NUM != magic_code {
      Err(OrcErr::MalformedOrcFormat(
        format!("Malformed ORC file. Invalid magic number {}",
        str::from_utf8(magic_code).expect("from_utf8()"))))      
    } else {
      Ok(())
    }    
  }
}

#[inline]
fn bytes_to_proto<P>(buf: &[u8], proto: P) -> OrcResult<P> where P: Message {
  compressed_bytes_to_proto(buf, proto, CompressionKind::NONE)
}

fn compressed_bytes_to_proto<P>(buf: &[u8], mut proto: P, codec: CompressionKind)
    -> OrcResult<P> where P: Message {  
  if codec == CompressionKind::NONE {
    let mut coded_stream = CodedInputStream::from_bytes(&buf);    
    proto.merge_from(&mut coded_stream).expect("merge_from()");
  } else {
    let mut slice = &buf[..];
    let mut codec_reader = OrcCompressReader::new(&mut slice, codec);
    let mut coded_stream = CodedInputStream::new(&mut codec_reader);
    proto.merge_from(&mut coded_stream).expect("merge_from()");
  }

  Ok(proto)
}

#[cfg(test)]
pub mod tests {
  use std::error::Error;
  use super::OrcReader;
  use fs::{FileReader, StreamReader};
  use version::*;

  fn open_file(path: &str) -> Box<StreamReader> {
    match FileReader::open(path) {
      Ok(r) => Box::new(r),
      Err(e) => panic!("{}", e.description())
    }
  }

  fn assert_file_version(path: &str, version: &Version) {
    let r = OrcReader::open(open_file(path)).expect(&format!("{} open failed", path));
    assert_eq!(r.format_version(), version);
  }

  fn assert_types(path: &str, types_str: &str) {
    let r = OrcReader::open(open_file(path)).expect(&format!("{} open failed", path));
    assert_eq!(&format!("{}", r.types()), types_str);
  }

  fn assert_orc_open(path: &str) {
    assert!(OrcReader::open(open_file(path)).is_ok());
  }

  #[test]
  fn test_versions() {
    assert_file_version("examples/orc-file-11-format.orc", &V_0_11);
    assert_file_version("examples/demo-12-zlib.orc", &V_0_12);
  }

  #[test]
  fn test_types() {
    assert_types("examples/orc-file-11-format.orc",
    "struct<boolean1:boolean,byte1:tinyint,short1:smallint,int1:int,\
    long1:bigint,float1:float,double1:double,bytes1:binary,string1:string,\
    middle:struct<list:array<struct<int1:int,string1:string>>>,\
    list:array<struct<int1:int,string1:string>>,\
    map:map<string,struct<int1:int,string1:string>>,ts:timestamp,decimal1:decimal(10,38)>");
  }

  #[test]
  fn test_none_v0_11() {
    assert_orc_open("examples/orc-file-11-format.orc");
  }

  // TODO - opening error
  //#[test]
  fn test_zlib_v0_11() {
    assert_orc_open("examples/demo-11-zlib.orc");
  }

  #[test]
  fn test_zlib_v0_12() {
    assert_orc_open("examples/demo-12-zlib.orc");
  }

  #[test]
  fn test_snappy() {
    assert_orc_open("examples/TestOrcFile.testSnappy.orc");
  }
}