use super::{OrcErr, OrcResult};
use fs::{FileInputStream, InputStream};
use orc_proto::{Footer, PostScript};

use std::cmp;
use std::str;
use protobuf::{CodedInputStream, Message};

pub struct OrcReader {

}

pub struct OrcTail {
}

impl OrcTail {
  pub fn empty() -> OrcTail {
    OrcTail {}
  }
}

const DIRECTORY_SIZE_GUESS: usize = 16 * 1024;

static MAGIC_NUM: &'static [u8;3] = b"ORC";
static MAGIC_NUM_LEN: usize = 3;
static LEN_OF_POST_SCRIPT_LEN: usize = 1;

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
    let ps: PostScript = OrcReader::extract_postscript(&buf, ps_offset, ps_len)?;
    let footer_offset = ps_offset - ps.get_footerLength() as usize;
    debug!("buf len: {}", buf.len());
    debug!("ps offset: {}", ps_offset);
    debug!("ps len: {}", ps_len);
    debug!("footer offset: {}", footer_offset);
    debug!("footer len: {}", ps.get_footerLength());
    let footer: Footer = OrcReader::extract_footer(&buf, footer_offset, ps.get_footerLength() as usize)?;
    Ok(OrcTail {})
  }

  pub fn ensure_footer(buf: &[u8], ps_len: usize) -> OrcResult<()> {
    let magic_len = (MAGIC_NUM_LEN + LEN_OF_POST_SCRIPT_LEN) as usize; // length of magic number + post script length

    if ps_len < magic_len {
      return Err(OrcErr::MalformedFormat(
        format!("Malformed ORC file. Invalid postscript length {}", ps_len)));
    }

    let magic_offset = buf.len() - magic_len;
    let magic_code = &buf[magic_offset..(magic_offset + MAGIC_NUM_LEN)];

    if MAGIC_NUM != magic_code {
      Err(OrcErr::MalformedFormat(
        format!("Malformed ORC file. Invalid magic number {}",
        str::from_utf8(magic_code).expect("from_utf8()"))))      
    } else {
      Ok(())
    }    
  }

  fn extract_postscript(buf: &[u8], abs_offset: usize, len: usize) 
      -> OrcResult<PostScript> {
    let ps_buf = &buf[abs_offset .. abs_offset + len];
    let mut coded_stream = CodedInputStream::from_bytes(&ps_buf);
    let mut ps = PostScript::new();
    ps.merge_from(&mut coded_stream).expect("PostScript::merge_from()");
    Ok(ps)
  }

  fn extract_footer(buf: &[u8], abs_offset: usize, len: usize) -> OrcResult<Footer> {
    let footer_buf = &buf[abs_offset .. abs_offset + len];
    let mut coded_stream = CodedInputStream::from_bytes(&footer_buf);
    let mut footer = Footer::new();
    footer.merge_from(&mut coded_stream).expect("Footer::merge_from()");
    Ok(footer)
  }
}

#[test]
fn test_tail() {
  use env_logger;
  env_logger::init();
  assert!(OrcReader::extract_tail("examples/orc-file-11-format.orc").is_ok());
}