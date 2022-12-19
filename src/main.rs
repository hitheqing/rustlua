extern crate core;

use std::fs::File;
use std::io::Read;

use crate::bin_struct::luabinstruct::{dump, ProtoType};

mod bin_struct;

fn main() {
    let mut f = File::open("luac.out").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let proto_type = dump(buf.as_slice());
    if let Some(p) = proto_type {
        eprint!("{:?}", p);
        list(&p);
    }
}

fn list(p: &ProtoType) {
    print_header(p);
    print_code(p);
    print_details(p);
    for x in &p.protos {
        list(x)
    }
}

fn print_details(p: &ProtoType) {
    println!("constants ({})", p.constants.len());
    for (i, x) in p.constants.iter().enumerate() {
        println!("\t{}\t\"{}\"", i + 1, x.to_str());
    }
    println!("locals ({})", p.loc_vars.len());
    for (i, x) in p.loc_vars.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}", i + 1, x.var_name, x.start_pc + 1, x.end_pc + 1);
    }
    println!("upvalues ({})", p.upval.len());
    for (i, x) in p.upval.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}", i + 1, p.upvalue_names[i], x.instack, x.idx);
    }
}

fn print_code(p: &ProtoType) {
    for (c, x) in p.code.iter().enumerate() {
        let mut line = String::new();
        if !p.line_info.is_empty() {
            line = format!("{}", p.line_info[c]);
        }
        // {:X} 按大写hex打印。{:08X}打印8位数字，前面补0
        println!("\t{}\t[{}]\t0x{:08X}", c + 1, line, x);
    }
}

fn print_header(p: &ProtoType) {
    let mut name = "main";
    if p.line_defined > 0 {
        name = "function";
    }
    let mut var_arg_flag = "";
    if p.is_var_arg > 0 {
        var_arg_flag = "+";
    }
    println!("\n{} <{}:{},{}> ({} instructions)", name, p.source, p.line_defined, p.last_line_defined, p.code.len());
    println!("{}{} params, {} slots, {} upvalues, {} locals,{} constants, {} functions", p.num_params, var_arg_flag, p.stack_size, p.upval.len(), p.loc_vars.len(), p.constants.len(), p.protos.len());
}
