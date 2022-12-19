use std::str::FromStr;
use super::reader::Reader;

const magic_bytes: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61];
const version: u8 = 0x53;
const format: u8 = 0;
const data2: [u8; 6] = [0x19, 0x93, 0x0D, 0x0A, 0x1A, 0x0A];
const cint_size: u8 = 4;
const csize_t_size: u8 = 8;
const instruction_size: u8 = 4;
const lua_integer_size: u8 = 8;
const lua_number_size: u8 = 8;
const lua_integer: i64 = 0x5678;
const lua_number: f64 = 370.5;

const t_nil: u8 = 0;
const t_boolean: u8 = 1;
const t_number: u8 = 3;
const t_sstring: u8 = 4;
const t_integer: u8 = 0x13;
const t_lstring: u8 = 0x14;

enum tag {
    t_nil = 0,
    t_boolean = 1,
    t_number = 3,
    t_sstring = 4,
    t_integer = 0x13,
    t_lstring = 0x14,
}


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
}

#[derive(Debug)]
pub struct ProtoType {
    pub source: String,
    pub lineDefined: u32,
    pub lastLineDefined: u32,
    pub numParams: u8,
    pub isVarArg: u8,
    pub StackSize: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upval: Vec<Upvalue>,
    pub protos: Vec<ProtoType>,
    pub lineInfo: Vec<u32>,
    pub locVars: Vec<LocVar>,
    pub upvalueNames: Vec<String>,
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
    pub(crate) fn ToString(&self) -> String {
        match self {
            Constant::Nil => "nil".to_string(),
            Constant::Boolean(b) => if *b { "true".to_string()}else { "false".to_string() },
            Constant::Number(n) => n.to_string(),
            Constant::Integer(i) => i.to_string(),
            Constant::SString(s) => s.to_string(),
            Constant::LString(s) => s.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Upvalue {
    pub Instack: u8,
    pub Idx: u8,
}
#[derive(Debug)]
pub struct LocVar {
    pub VarName: String,
    pub StartPC: u32,
    pub EndPC: u32,
}

pub fn dump(data: &[u8]) -> ProtoType{
    let mut reader = Reader::new(data);
    reader.read_header();
    reader.read_byte();
    reader.read_proto("")
}

impl<'a> Reader<'a> {

    pub fn read_header(&mut self) -> bool {
        if self.read_bytes(4) != magic_bytes {
            println!("magic_bytes not matched!");
            return false;
        }
        if self.read_byte() != version {
            println!("version not matched!");
            return false;
        }
        if self.read_byte() != format {
            println!("format not matched!");
            return false;
        }
        if self.read_bytes(6) != data2 {
            println!("data2 not matched!");
            return false;
        }
        if self.read_byte() != cint_size {
            println!("cint_size not matched!");
            return false;
        }
        if self.read_byte() != csize_t_size {
            println!("csize_t_size not matched!");
            return false;
        }
        if self.read_byte() != instruction_size {
            println!("instruction_size not matched!");
            return false;
        }
        if self.read_byte() != lua_integer_size {
            println!("lua_integer_size not matched!");
            return false;
        }
        if self.read_byte() != lua_number_size {
            println!("lua_number_size not matched!");
            return false;
        }
        if self.read_i64() != lua_integer {
            println!("lua_magic_num not matched!");
            return false;
        }
        if self.read_f64() != lua_number {
            println!("lua_magic_float not matched!");
            return false;
        }
        println!("parse head succ!");
        true
    }



    fn read_code(&mut self) -> Vec<u32> {
        let mut vec = vec![];
        let len = self.read_u32();
        for i in 0..len {
            vec.push(self.read_u32());
        }
        vec
    }

    fn read_tag(&mut self) -> tag {
        match self.read_byte() {
            t_boolean => tag::t_boolean,
            t_number => tag::t_number,
            t_sstring => tag::t_sstring,
            t_integer => tag::t_integer,
            t_lstring => tag::t_lstring,
            _ => tag::t_nil,
        }
    }

    fn read_constant(&mut self) -> Constant {
        match self.read_tag() {
            tag::t_nil => Constant::Nil,
            tag::t_boolean =>Constant::Boolean(self.read_byte() != 0),
            tag::t_number => Constant::Number(self.read_f64()),
            tag::t_sstring => Constant::SString(self.read_str()),
            tag::t_integer => Constant::Integer(self.read_i64()),
            tag::t_lstring => Constant::LString(self.read_str()),
        }
    }

    fn read_upvalue(&mut self) -> Vec<Upvalue> {
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(Upvalue{
                Instack:self.read_byte(),
                Idx:self.read_byte(),
            })
        }
        vec
    }
    fn read_line_info(&mut self) -> Vec<u32> {
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(self.read_u32())
        }
        vec
    }
    fn read_local_vars(&mut self) -> Vec<LocVar> {
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(LocVar{
                VarName:self.read_str(),
                StartPC:self.read_u32(),
                EndPC:self.read_u32(),
            })
        }
        vec
    }
    fn read_upvalue_names(&mut self) -> Vec<String> {
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(self.read_str())
        }
        vec
    }
    fn read_constans(&mut self) -> Vec<Constant> {
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(self.read_constant())
        }
        vec
    }

    fn read_protos(&mut self,parentSource:&str) -> Vec<ProtoType> {
        self.truncate();
        let len = self.read_u32();
        let mut vec = vec![];
        for i in 0..len {
            vec.push(self.read_proto(parentSource))
        }
        vec
    }

    fn read_proto(&mut self,parentSource:&str) -> ProtoType {
        let mut source = self.read_str();
        if source.as_str() == "" {
            source = parentSource.to_string()   ;
        }
        // ProtoType{
            let a1 = source;
            let a2 =  self.read_u32();
            let a3 =  self.read_u32();
            let a4 =  self.read_byte();
            let a5 =  self.read_byte();
            let a6 = self.read_byte();
            let a7 =  self.read_code();
            let a8 =  self.read_constans();
            let a9 =  self.read_upvalue();
            let a10 =  self.read_protos(parentSource);
            let a11 = self.read_line_info();
            let a12 =  self.read_local_vars();
            let a13 =  self.read_upvalue_names();
        // }
        ProtoType{
            source:a1 ,
            lineDefined:a2 ,
            lastLineDefined:a3 ,
            numParams:a4 ,
            isVarArg:a5 ,
            StackSize:a6 ,
            code:a7 ,
            constants: a8 ,
            upval: a9 ,
            protos: a10,
            lineInfo: a11,
            locVars: a12,
            upvalueNames: a13,
        }

    }
}