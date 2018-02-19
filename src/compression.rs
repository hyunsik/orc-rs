use std::io::{self, Read};
use flate2::read::DeflateDecoder;
use orc_proto::CompressionKind;
use snap;

pub struct OrcCompressReader<'a> {
  reader: &'a mut Read,
  hd_buf: [u8; 3],
  codec: CompressionKind
}

impl<'a> OrcCompressReader<'a> {
  pub fn new(r: &'a mut Read, codec: CompressionKind) -> OrcCompressReader {
    OrcCompressReader {
      reader: r,
      hd_buf: [0; 3],
      codec: codec
    }
  }
}

impl<'a> Read for OrcCompressReader<'a> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    debug!("OrcCompressReader::read() enter!");

    match self.reader.take(3).read(&mut self.hd_buf) {
      Ok(0) => {
        debug!("OrcCompressReader::read() eof!");
        return Ok(0) // EOF
      }
      Ok(3) => {
        debug!("OrcCompressReader::read() header is ok!")
      }
      Ok(_) => panic!("invalid compression block header"),
      Err(e) => {
        debug!("{}", e);
        return Err(e)
      }
    };

    let mut header: u32 = self.hd_buf[0] as u32;
    header |= (self.hd_buf[1] as u32) << 8;
    header |= (self.hd_buf[2] as u32) << 16;
    let is_origin = (header & 0x01) == 1;
    let chunk_len = (header >> 1) as usize;
    debug!("is_origin: {}", is_origin);
    debug!("chunk_len: {}", chunk_len);    

    if is_origin {
      Ok(self.reader.take(chunk_len as u64).read(buf)?)
    } else {    
      let mut chunk: Vec<u8> = Vec::with_capacity(chunk_len);
      self.reader.take(chunk_len as u64).read_to_end(&mut chunk)?;

      let uncompressed_len = match self.codec {
        CompressionKind::SNAPPY => {          
          let mut d = snap::Decoder::new();          
          d.decompress(&chunk[..chunk_len], buf).expect("snap::decompress")
        }
        CompressionKind::ZLIB => {
          let mut d = DeflateDecoder::new(&chunk[..chunk_len]);
          d.read(buf).expect("DeflateDecoder::read()")
        }
        _ => panic!("unsupported compression codec {:?}", self.codec)
      };

      debug!("uncompressed {} bytes", uncompressed_len);
      Ok(uncompressed_len)
    }
  }
}