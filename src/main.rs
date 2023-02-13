mod api;
mod binary;
mod state;
mod vm;
use crate::api::{consts::*, LuaAPI, LuaVM};
use crate::binary::chunk::Prototype;
use crate::state::LuaState;
use crate::vm::instruction::Instruction;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

/*
code:
local t = {"a","b","c"}
t[2] = "B"
t["foo"] = "Bar"
local s = t[3]..t[2]..t[1]..t["foo"]..#t

terminal:
[0001] NEWTABLE [table][nil][nil][nil][nil][nil]
[0002] LOADK    [table]["a"][nil][nil][nil][nil]
[0003] LOADK    [table]["a"]["b"][nil][nil][nil]
[0004] LOADK    [table]["a"]["b"]["c"][nil][nil]
[0005] SETLIST  [table]["a"]["b"]["c"][nil][nil]
[0006] SETTABLE [table]["a"]["b"]["c"][nil][nil]
[0007] SETTABLE [table]["a"]["b"]["c"][nil][nil]
[0008] GETTABLE [table]["c"]["b"]["c"][nil][nil]
[0009] GETTABLE [table]["c"]["B"]["c"][nil][nil]
[0010] GETTABLE [table]["c"]["B"]["a"][nil][nil]
[0011] GETTABLE [table]["c"]["B"]["a"]["Bar"][nil]
[0012] LEN      [table]["c"]["B"]["a"]["Bar"][3]
[0013] CONCAT   [table]["cBaBar3"]["B"]["a"]["Bar"][3]
*/

fn main() -> io::Result<()> {
    if env::args().count() > 1 {
        let filename = env::args().nth(1).unwrap();
        let mut file = File::open(filename)?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        let proto = binary ::undump(data);
        lua_main(proto);
    }
    Ok(())
}

fn lua_main(proto: Prototype) {
    let nregs = proto.max_stack_size;
    let mut ls = state::new_lua_state((nregs + 8) as usize, proto);
    ls.set_top(nregs as isize);
    loop {
        let pc = ls.pc();
        let instr = ls.fetch();
        if instr.opcode() != vm::opcodes::OP_RETURN {
            instr.execute(&mut ls);
            print!("[{:04}] {} ", pc + 1, instr.opname());
            print_stack(&ls);
        } else {
            break;
        }
    }
}

fn print_stack(ls: &LuaState) {
    let top = ls.get_top();
    for i in 1..top + 1 {
        let t = ls.type_id(i);
        match t {
            LUA_TBOOLEAN => print!("[{}]", ls.to_boolean(i)),
            LUA_TNUMBER => print!("[{}]", ls.to_number(i)),
            LUA_TSTRING => print!("[{:?}]", ls.to_string(i)),
            _ => print!("[{}]", ls.type_name(t)), // other values
        }
    }
    println!("");
}
