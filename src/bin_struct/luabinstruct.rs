use std::fmt::{Display, Formatter};

use super::reader::Reader;

const T_NIL: u8 = 0;
const T_BOOLEAN: u8 = 1;
const T_NUMBER: u8 = 3;
const T_SSTRING: u8 = 4;
const T_INTEGER: u8 = 0x13;
const T_LSTRING: u8 = 0x14;

enum Tag {
    Nil = 0,
    Boolean = 1,
    Number = 3,
    ShortString = 4,
    Integer = 0x13,
    LongString = 0x14,
}

#[derive(PartialEq)]
struct Header {
    signature: [u8; 4],
    version: u8,
    format: u8,
    luac_data: [u8; 6],
    cint_size: u8,
    size_t_size: u8,
    instruction_size: u8,
    integer_size: u8,
    number_size: u8,
    lua_integer: i64,
    lua_number: f64,
}

impl Header {
    pub fn new_default() -> Self {
        Self {
            signature: [0x1b, 0x4c, 0x75, 0x61],
            version: 0x53,
            format: 0,
            luac_data: [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A],
            cint_size: 4,
            size_t_size: 8,
            instruction_size: 4,
            integer_size: 8,
            number_size: 8,
            lua_integer: 0x5678,
            lua_number: 370.5,
        }
    }

    pub fn new(
        signature: [u8; 4],
        version: u8,
        format: u8,
        luac_data: [u8; 6],
        cint_size: u8,
        size_t_size: u8,
        instruction_size: u8,
        integer_size: u8,
        number_size: u8,
        lua_integer: i64,
        lua_number: f64,
    ) -> Self {
        Self {
            signature,
            version,
            format,
            luac_data,
            cint_size,
            size_t_size,
            instruction_size,
            integer_size,
            number_size,
            lua_integer,
            lua_number,
        }
    }
}

#[derive(Debug)]
pub struct ProtoType {
    pub source: String,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_var_arg: u8,
    pub stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upval: Vec<Upvalue>,
    pub protos: Vec<ProtoType>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<String>,
}

#[derive(Debug)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    SString(String),
    LString(String),
}

impl Constant {
    pub(crate) fn to_str(&self) -> String {
        match self {
            Constant::Nil => "nil".to_string(),
            Constant::Boolean(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Constant::Number(n) => n.to_string(),
            Constant::Integer(i) => i.to_string(),
            Constant::SString(s) => s.to_string(),
            Constant::LString(s) => s.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Upvalue {
    pub instack: u8,
    pub idx: u8,
}

#[derive(Debug)]
pub struct LocVar {
    pub var_name: String,
    pub start_pc: u32,
    pub end_pc: u32,
}

pub fn dump(data: &[u8]) -> Option<ProtoType> {
    let mut reader = Reader::new(data);
    if reader.read_header() {
        reader.read_byte();
        return Some(reader.read_proto(""));
    }
    None
}

impl<'a> Reader<'a> {
    pub fn read_header(&mut self) -> bool {
        let header = Header::new(
            self.read_4(),
            self.read_byte(),
            self.read_byte(),
            self.read_6(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_i64(),
            self.read_f64(),
        );

        let def = Header::new_default();
        def.eq(&header)
    }

    fn read_code(&mut self) -> Vec<u32> {
        let mut vec = vec![];
        let len = self.read_u32();
        for _i in 0..len {
            vec.push(self.read_u32());
        }
        vec
    }

    fn read_tag(&mut self) -> Tag {
        match self.read_byte() {
            T_BOOLEAN => Tag::Boolean,
            T_NUMBER => Tag::Number,
            T_SSTRING => Tag::ShortString,
            T_INTEGER => Tag::Integer,
            T_LSTRING => Tag::LongString,
            _ => Tag::Nil,
        }
    }

    fn read_constant(&mut self) -> Constant {
        match self.read_tag() {
            Tag::Nil => Constant::Nil,
            Tag::Boolean => Constant::Boolean(self.read_byte() != 0),
            Tag::Number => Constant::Number(self.read_f64()),
            Tag::ShortString => Constant::SString(self.read_string()),
            Tag::Integer => Constant::Integer(self.read_i64()),
            Tag::LongString => Constant::LString(self.read_string()),
        }
    }

    fn read_upvalue(&mut self) -> Vec<Upvalue> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(Upvalue {
                instack: self.read_byte(),
                idx: self.read_byte(),
            })
        }
        vec
    }
    fn read_line_info(&mut self) -> Vec<u32> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(self.read_u32())
        }
        vec
    }
    fn read_local_vars(&mut self) -> Vec<LocVar> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(LocVar {
                var_name: self.read_string(),
                start_pc: self.read_u32(),
                end_pc: self.read_u32(),
            })
        }
        vec
    }
    fn read_upvalue_names(&mut self) -> Vec<String> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(self.read_string())
        }
        vec
    }
    fn read_constans(&mut self) -> Vec<Constant> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(self.read_constant())
        }
        vec
    }

    fn read_protos(&mut self, parent_source: &str) -> Vec<ProtoType> {
        let len = self.read_u32();
        let mut vec = vec![];
        for _i in 0..len {
            vec.push(self.read_proto(parent_source))
        }
        vec
    }

    fn read_proto(&mut self, parent_source: &str) -> ProtoType {
        let mut source = self.read_string();
        if source.as_str() == "" {
            source = parent_source.to_string();
        }
        // ProtoType{
        let a1 = source;
        let a2 = self.read_u32();
        let a3 = self.read_u32();
        let a4 = self.read_byte();
        let a5 = self.read_byte();
        let a6 = self.read_byte();
        let a7 = self.read_code();
        let a8 = self.read_constans();
        let a9 = self.read_upvalue();
        let a10 = self.read_protos(parent_source);
        let a11 = self.read_line_info();
        let a12 = self.read_local_vars();
        let a13 = self.read_upvalue_names();
        // }
        ProtoType {
            source: a1,
            line_defined: a2,
            last_line_defined: a3,
            num_params: a4,
            is_var_arg: a5,
            stack_size: a6,
            code: a7,
            constants: a8,
            upval: a9,
            protos: a10,
            line_info: a11,
            loc_vars: a12,
            upvalue_names: a13,
        }
    }
}
