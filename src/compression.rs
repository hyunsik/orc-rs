use std::io::{self, Read};
use flate2::read::ZlibDecoder;
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

    match self.reader.read(&mut self.hd_buf) {
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

    let mut chunk: Vec<u8> = Vec::with_capacity(chunk_len);  
    unsafe { chunk.set_len(chunk_len); };
    debug!("read_to_end before");
    self.reader.read_exact(&mut chunk)?;
    debug!("read_to_end after");

    if is_origin {
      buf[..chunk_len].copy_from_slice(&chunk);
      Ok(chunk_len)
    } else {    
      let uncompressed_len = match self.codec {
        CompressionKind::SNAPPY => {
          let estimated_len = snap::decompress_len(&chunk[..chunk_len]).expect("snap::decompress_len");
          let mut d = snap::Decoder::new();
          let mut uncompress_buf = Vec::with_capacity(estimated_len);
          unsafe { uncompress_buf.set_len(estimated_len)};
          let uncompressed_len = d.decompress(&chunk[..chunk_len], &mut uncompress_buf)
            .expect("snap::Decoder::decompress()");
          buf[..uncompressed_len].copy_from_slice(&uncompress_buf[..uncompressed_len]);
          uncompressed_len
        }
        CompressionKind::ZLIB => {
          let mut d = ZlibDecoder::new(&chunk[0..chunk_len]);
          d.read(buf).expect("ZlibDecoder::read()")
        }
        _ => panic!("unsupported compression codec {:?}", self.codec)
      };

      debug!("uncompressed {} bytes", uncompressed_len);
      Ok(uncompressed_len)
    }
  }
}