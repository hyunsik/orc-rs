extern crate log;
extern crate env_logger;

extern crate protoc_rust;

use std::path::Path;
use protoc_rust::Args;

fn main() {
  env_logger::init();
  let path = Path::new("src/orc_proto.rs");
  if !path.exists() {
    let protoc_arg = Args {
      out_dir: "src",
      includes: &["proto"],
      input: &["proto/orc_proto.proto"],
    };  
    protoc_rust::run(protoc_arg).expect("protoc");
  }  
}