use super::{OrcErr, OrcResult};
use compression::OrcCompressReader;
use fs::{FileInputStream, InputStream};
use orc_proto::{CompressionKind, Footer, PostScript};

use std::cmp;
use std::str;
use protobuf::{CodedInputStream, Message};


static MAGIC_NUM: &'static [u8;3] = b"ORC";
static MAGIC_NUM_LEN: usize = 3;
static LEN_OF_POST_SCRIPT_LEN: usize = 1;
const DIRECTORY_SIZE_GUESS: usize = 16 * 1024;

pub struct OrcRecordReader {  
}

pub struct OrcReader {
}

pub struct OrcTail {
}

impl OrcTail {
  pub fn empty() -> OrcTail {
    OrcTail {}
  }
}

impl OrcReader {
  pub fn extract_tail(path: &str) -> OrcResult<OrcTail> {
    let mut fis = FileInputStream::open(path)?;
    let mut buf: [u8; DIRECTORY_SIZE_GUESS] = [0; DIRECTORY_SIZE_GUESS];
    let size = fis.len();

    // read last bytes into buffer to get PostScript
    let read_size = cmp::min(size, DIRECTORY_SIZE_GUESS as u64);
    fis.read_at(&mut buf, size - read_size)?;

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
    Ok(OrcTail {})
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
pub fn bytes_to_proto<P>(buf: &[u8], proto: P) -> OrcResult<P> where P: Message {
  compressed_bytes_to_proto(buf, proto, CompressionKind::NONE)
}

pub fn compressed_bytes_to_proto<P>(buf: &[u8], mut proto: P, codec: CompressionKind)
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

#[test]
fn test_none() {
  assert!(OrcReader::extract_tail("examples/orc-file-11-format.orc").is_ok());
}

#[test]
fn test_zlib() {
  assert!(OrcReader::extract_tail("examples/demo-12-zlib.orc").is_ok());
}

#[test]
fn test_snappy() {
  assert!(OrcReader::extract_tail("examples/TestOrcFile.testSnappy.orc").is_ok());
}