use std::i32;

/// a version number for the ORC file format
#[derive(Clone, Copy, Hash, PartialEq)]
pub struct Version(i32, i32, &'static str);

pub static V_0_11: Version = Version(0, 11, "0.11");
pub static V_0_12: Version = Version(0, 12, "0.12");
/// Do not use this format except for testing. It will not be compatible
/// with other versions of the software. While we iterate on the ORC 2.0
/// format, we will make incompatible format changes under this version
/// without providing any forward or backward compatibility.
///
/// When 2.0 is released, this version identifier will be completely removed.
pub static UNSTABLE_PRE_2_0: Version = Version(1, 9999, "UNSTABLE-PRE-2.0");
/// The generic identifier for all unknown versions.
pub static FURTURE: Version = Version(i32::MAX, i32::MAX, "future");

fn find(versions: &[i32]) -> Version {
  assert!(versions.len() == 2, 
    "wrong version: versions has {} numbers", versions.len());
  
  match (versions[0],versions[1]) {
    (0, 11) => V_0_11,
    (0, 12) => V_0_12,
    (1, 9999) => UNSTABLE_PRE_2_0,
    (i32::MAX, i32::MAX) => FURTURE,
    // Others are not intended
    _ => panic!("Unknown ORC version: major ({}), minor ({})", versions[0], versions[0])
  }
} 

pub mod writer {
  use std::i32;

  pub struct WriterVersion (i32, i32);

  ///////////////////////////////////
  // Writer Implementations
  ///////////////////////////////////
  /// ORC Java Writer
  pub const JAVA_IMPL: i32 = 0;
  /// ORC C++ Writer
  pub const CPP_IMPL: i32 = 1;
  /// Presto Writer
  pub const PRESTO_IMPL: i32 = 2;
  /// Go writer from https://github.com/scritchley/orc
  pub const SCRITCHLEY_GO_IMPL: i32 = 3;
  pub const UNKNOWN_IMPL: i32 = i32::MAX;

  ///////////////////////////////////
  // Java ORC Writer
  ///////////////////////////////////
  pub static ORIGINAL: WriterVersion = WriterVersion(JAVA_IMPL, 0);
  // fixed stripe/file maximum, statistics & string statistics 
  // and use utf8 for min/max
  pub static HIVE_8732: WriterVersion = WriterVersion(JAVA_IMPL, 1);
  // use real column names from Hive tables
  pub static HIVE_4243: WriterVersion = WriterVersion(JAVA_IMPL, 2);
  // vectorized writer
  pub static HIVE_12055: WriterVersion = WriterVersion(JAVA_IMPL, 3);
  // decimals write present stream correctly
  pub static HIVE_13083: WriterVersion = WriterVersion(JAVA_IMPL, 4);
  // bloom filters use utf8
  pub static ORC_101: WriterVersion = WriterVersion(JAVA_IMPL, 5);
  // timestamp stats use utc
  pub static ORC_135: WriterVersion = WriterVersion(JAVA_IMPL, 6);

  /// C++ ORC Writer
  pub static ORC_CPP_ORIGIN: WriterVersion = WriterVersion(CPP_IMPL, 6);

  ///////////////////////////////////
  // Other Writers
  ///////////////////////////////////
  pub static PRESTO_ORIGIN: WriterVersion = WriterVersion(PRESTO_IMPL, 6);
  /// Scritchley Go Writer
  pub static SCRITCHLEY_GO_ORIGINAL: WriterVersion = WriterVersion(SCRITCHLEY_GO_IMPL, 6);
  /// Don't use any magic numbers here except for the below:
  pub static FUTURE: WriterVersion = WriterVersion(UNKNOWN_IMPL, i32::MAX);
}