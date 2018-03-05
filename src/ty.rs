use std::fmt;
use itertools::Itertools;

const MAX_PRECISION: u32 = 38;
const MAX_SCALE: u32 = 38;
const DEFAULT_PRECISION: u32 = 38;
const DEFAULT_SCALE: u32 = 10;
const DEFAULT_LENGTH: u32 = 256;

use self::TypeKind::*;
use orc_proto::{self, Type_Kind};

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

  pub fn has_scale_or_precision(&self) -> bool {
    match *self {
      DECIMAL => true,
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
  List(Box<Type>),
  Map(Box<Type>, Box<Type>),
  Struct(Vec<Box<Type>>, Vec<String>),
  Union(Vec<Box<Type>>),
}

impl Type {
  pub fn new() -> Type {
    unimplemented!()   
  }

  pub fn from_proto(types: &[orc_proto::Type]) -> Type {
    Type::from_proto_(types, 0)
  }

  fn from_proto_(types: &[orc_proto::Type], root_idx: usize) -> Type {
    let root_ty = &types[root_idx];

    match root_ty.get_kind() {
      Type_Kind::BOOLEAN => Type::create_bool(),
      Type_Kind::BYTE => Type::create_byte(),
      Type_Kind::SHORT => Type::create_short(),
      Type_Kind::INT => Type::create_int(),
      Type_Kind::LONG => Type::create_long(),
      Type_Kind::FLOAT => Type::create_float(),
      Type_Kind::DOUBLE => Type::create_double(),
      Type_Kind::STRING => Type::create_string(),
      Type_Kind::CHAR | Type_Kind::VARCHAR => {        
        let len = if root_ty.has_maximumLength() {
          root_ty.get_maximumLength()
        } else {
          DEFAULT_LENGTH
        };

        match root_ty.get_kind() {
          Type_Kind::CHAR => Type::create_char_with(len),
          Type_Kind::VARCHAR => Type::create_varchar_with(len),
          _ => unreachable!()
        }        
      }
      Type_Kind::BINARY => Type::create_binary(),
      Type_Kind::TIMESTAMP => Type::create_timestamp(),
      Type_Kind::DATE => Type::create_date(),

      Type_Kind::DECIMAL => {
        let scale = if root_ty.has_scale() {
          root_ty.get_scale()
        } else {
          DEFAULT_SCALE
        };
        let precision = if root_ty.has_precision() {
          root_ty.get_precision()
        } else {
          DEFAULT_PRECISION
        };

        Type::create_decimal_with(scale, precision)
      }

      Type_Kind::LIST => {
        Type::create_list(Type::from_proto_(types, root_ty.get_subtypes()[0] as usize))
      }
      Type_Kind::MAP => {
        Type::create_map(
          Type::from_proto_(types, root_ty.get_subtypes()[0] as usize),
          Type::from_proto_(types, root_ty.get_subtypes()[1] as usize)
        )
      }

      Type_Kind::STRUCT => {
        debug_assert_eq!(root_ty.get_fieldNames().len(), root_ty.get_subtypes().len(),
          "the number of subtypes and and fieldNames are mismatched to each other.");
        
        let mut struct_ty = Type::create_struct();      
        root_ty.get_fieldNames().iter()
          .zip(root_ty.get_subtypes().iter())
          .for_each(|(name, subty_idx)| {
            struct_ty.add_field(name, Type::from_proto_(types, *subty_idx as usize));
          });

        struct_ty
      }

      Type_Kind::UNION => {
        let mut union_ty = Type::create_union();
        root_ty.get_subtypes().iter()
          .for_each(|subty_idx| {
            union_ty.add_union_child(Type::from_proto_(types, *subty_idx as usize));
          });

        union_ty
      }
    }
  }

  pub fn kind(&self) -> TypeKind {
    match *self {
      Type::Primitive(kind) => kind, 
      Type::PrimitiveWithLen(kind, _) => kind, 
      Type::ScalePrimitive(kind, _, _) => kind,
      Type::List(_) => LIST,
      Type::Map(_,_) => MAP,      
      Type::Struct(_, _) => STRUCT,
      Type::Union(_) => UNION,
    }
  }

  pub fn create_bool() -> Self {
    Type::create_primitive(BOOLEAN)
  }

  pub fn create_byte() -> Self {
    Type::create_primitive(BYTE)
  }

  pub fn create_short() -> Self {
    Type::create_primitive(SHORT)
  }

  pub fn create_int() -> Self {
    Type::create_primitive(INT)
  }

  pub fn create_long() -> Self {
    Type::create_primitive(LONG)
  }

  pub fn create_float() -> Self {
    Type::create_primitive(FLOAT)
  }

  pub fn create_double() -> Self {
    Type::create_primitive(DOUBLE)
  }

  pub fn create_string() -> Self {
    Type::create_primitive(STRING)
  }

  pub fn create_primitive(kind: TypeKind) -> Self {
    assert!(kind.is_primitive(), "{} is not primitive type", kind);    
    Type::Primitive(kind)
  }

  pub fn create_char() -> Self {
    Type::PrimitiveWithLen(CHAR, DEFAULT_LENGTH)
  }

  pub fn create_char_with(len: u32) -> Self {
    assert!(len > 0, "lenth {} must be a positive integer", len);
    Type::PrimitiveWithLen(CHAR, len)
  }

  pub fn create_varchar() -> Self {        
    Type::PrimitiveWithLen(VARCHAR, DEFAULT_LENGTH)
  }

  pub fn create_varchar_with(len: u32) -> Self {
    assert!(len > 0, "lenth {} must be a positive integer", len);
    Type::PrimitiveWithLen(VARCHAR, len)
  }

  pub fn create_binary() -> Self {        
    Type::Primitive(BINARY)
  }
  
  pub fn create_timestamp() -> Self {        
    Type::Primitive(TIMESTAMP)
  }
  
  pub fn create_date() -> Self {        
    Type::Primitive(DATE)
  } 

  pub fn create_decimal() -> Self {
    Type::ScalePrimitive(DECIMAL, DEFAULT_PRECISION, DEFAULT_SCALE)
  }

  #[inline]
  pub fn create_decimal_with_precision(precision: u32) -> Self {   
    Type::create_decimal_with(precision, DEFAULT_SCALE)
  }

  pub fn create_decimal_with(precision: u32, scale: u32) -> Self {
    assert!(precision > 0 && precision <= MAX_PRECISION, "precision {} is out of range 1 .. {}", 
      precision, MAX_PRECISION);  
    assert!(scale > 0 && scale <= MAX_SCALE, "scale {} is out of range 1 .. {}", 
      scale, MAX_SCALE);

    Type::ScalePrimitive(DECIMAL, precision, scale)
  }

  pub fn create_list(elem_ty: Type) -> Self {
    Type::List(Box::new(elem_ty))
  }

  pub fn create_map(key_ty: Type, val_ty: Type) -> Self {
    Type::Map(Box::new(key_ty), Box::new(val_ty))
  }

  pub fn create_struct() -> Self {
    Type::Struct(Vec::new(), Vec::new())
  }

  pub fn add_field(&mut self, name: &str, subty: Type) -> &mut Self { 
    match *self {
      Type::Struct(ref mut child, ref mut fields) => {        
        child.push(Box::new(subty));
        fields.push(name.to_owned());
      }
      _ => panic!("Struct only allows add_field()")
    }
    self
  }

  pub fn create_union() -> Self {
    Type::Union(Vec::new())
  }

  pub fn add_union_child(&mut self, ty: Type) -> &mut Self {
    match *self {
      Type::Union(ref mut child) => {
        child.push(Box::new(ty));
      }
      _ => panic!("Union only allows add_union_child()")
    }

    self
  }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {      
      match *self {
        Type::Primitive(ref kind) => write!(f, "{}", kind),
        Type::PrimitiveWithLen(ref kind, len) => write!(f, "{}({})", kind, len),
        Type::ScalePrimitive(ref kind, prec, scale) => write!(f, "{}({},{})", kind, prec, scale),
        Type::List(ref elem_ty) => write!(f, "array<{}>", elem_ty),
        Type::Map(ref key_ty, ref val_ty) => write!(f, "map<{},{}>", key_ty, val_ty),
        Type::Struct(ref types, ref fields) => {
          debug_assert_eq!(types.len(), fields.len(),
            "the number of subtypes and and fieldNames are mismatched to each other.");

          let child = fields.iter().zip(types.iter())
            .map(|(name, ty)| {
              format!("{}:{}", name, ty)
            })
            .join(",");

          write!(f, "struct<{}>", child)
        },
        Type::Union(ref types) => {
          let child = types.iter().map(|t| format!("{}", t)).join(",");
          write!(f, "uniontype<{}>", child)
        }
      }
    }
}