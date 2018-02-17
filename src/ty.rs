use std::fmt;

const MAX_PRECISION: u32 = 38;
const MAX_SCALE: u32 = 38;
const DEFAULT_PRECISION: u32 = 38;
const DEFAULT_SCALE: u32 = 10;
const DEFAULT_LENGTH: u32 = 256;

use self::TypeKind::*;

#[derive(Copy,Clone,PartialEq,Eq,Debug,Hash)]
pub enum TypeKind {
    BOOLEAN = 0,
    BYTE = 1,
    SHORT = 2,
    INT = 3,
    LONG = 4,
    FLOAT = 5,
    DOUBLE = 6,
    STRING = 7,
    BINARY = 8,
    TIMESTAMP = 9,
    LIST = 10,
    MAP = 11,
    STRUCT = 12,
    UNION = 13,
    DECIMAL = 14,
    DATE = 15,
    VARCHAR = 16,
    CHAR = 17,
}

impl TypeKind {
  pub fn name(&self) -> &str {
    match *self {
      BOOLEAN => "boolean",
      BYTE => "tinyint",
      SHORT => "smallint",
      INT => "int",
      LONG => "bigint",
      FLOAT => "float",
      DOUBLE => "double",
      STRING => "string",
      BINARY => "binary",
      TIMESTAMP => "timestamp",
      LIST => "array",
      MAP => "map",
      STRUCT => "struct",
      UNION => "union",
      DECIMAL => "decimal",
      DATE => "date",
      VARCHAR => "varchar",
      CHAR => "char",
    }
  }

  pub fn is_primitive(&self) -> bool {
    match *self {
      LIST | STRUCT | MAP | UNION => false,
      _ => true
    }
  }

  pub fn has_length(&self) -> bool {
    match *self {
      VARCHAR | CHAR => true,
      _ => false
    }
  }
}

impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum Type {
  Primitive(TypeKind),
  PrimitiveWithLen(TypeKind, u32), // Varchar, Char
  ScalePrimitive(TypeKind, u32, u32), // Decimal  
  List(TypeKind, Box<Type>), // List
  Union(TypeKind, Vec<Box<Type>>), // Union
  Struct(TypeKind, Vec<Box<Type>>, Vec<String>) // Struct
}

impl Type {
  pub fn kind(&self) -> TypeKind {
    unimplemented!()
  }

  pub fn new_primitive(kind: TypeKind) -> Self {
    assert!(kind.is_primitive(), "{} is not primitive type", kind);    
    Type::Primitive(kind)
  }

  pub fn new_char() -> Self {
    Type::PrimitiveWithLen(CHAR, DEFAULT_LENGTH)
  }

  pub fn new_char_with(len: u32) -> Self {
    assert!(len > 0, "lenth {} must be a positive integer", len);
    Type::PrimitiveWithLen(CHAR, len)
  }

  pub fn new_varchar() -> Self {        
    Type::PrimitiveWithLen(VARCHAR, DEFAULT_LENGTH)
  }

  pub fn new_varchar_with(len: u32) -> Self {
    assert!(len > 0, "lenth {} must be a positive integer", len);
    Type::PrimitiveWithLen(VARCHAR, len)
  }

  pub fn new_struct() -> Self {
    Type::Struct(STRUCT, Vec::new(), Vec::new())
  }

  pub fn new_decimal() -> Self {
    Type::ScalePrimitive(DECIMAL, DEFAULT_PRECISION, DEFAULT_SCALE)
  }

  #[inline]
  pub fn new_decimal_with_precision(precision: u32) -> Self {   
    Type::new_decimal_with(precision, DEFAULT_SCALE)
  }

  pub fn new_decimal_with(precision: u32, scale: u32) -> Self {
    assert!(precision > 0 && precision <= MAX_PRECISION, "precision {} is out of range 1 .. {}", 
      precision, MAX_PRECISION);  
    assert!(scale > 0 && scale <= MAX_SCALE, "scale {} is out of range 1 .. {}", 
      scale, MAX_SCALE);

    Type::ScalePrimitive(DECIMAL, precision, scale)
  }

  pub fn add_field(mut self, name: &str, ty: Type) -> Self { 
    match self {
      Type::Struct(_, ref mut child, ref mut fields) => {        
        child.push(Box::new(ty));
        fields.push(name.to_owned());
      }
      _ => panic!("Struct only allows add_field()")
    }
    self
  }

  pub fn add_union_child(mut self, ty: Type) -> Self {
    match self {
      Type::Union(_, ref mut child) => {
        child.push(Box::new(ty));
      }
      _ => panic!("Union only allows add_union_child()")
    }
    self
  }
}