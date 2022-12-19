extern crate core;

use std::fs::File;
use std::io::Read;

mod bin_struct;

use bin_struct::reader::Reader;
use crate::bin_struct::luabinstruct::{dump, ProtoType};

fn main() {

    let mut f = File::open("luac.out").unwrap();
    let mut buf = vec![];
    f.read_to_end(&mut buf).unwrap();

    let proto_type = dump(buf.as_slice());
    eprint!("{:?}",proto_type);
    list(&proto_type);
}

fn list(p: &ProtoType) {
    printHeader(p);
    printCode(p);
    printDetails(p);
    for x in &p.protos {
        list(x)
    }
}

fn printDetails(p: &ProtoType) {
    println!("constants ({})",p.constants.len());
    for (i,x) in p.constants.iter().enumerate() {
        println!("\t{}\t\"{}\"",i+1,x.ToString());
    }
    println!("locals ({})",p.locVars.len());
    for (i,x) in p.locVars.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}",i+1,x.VarName,x.StartPC+1,x.EndPC+1);
    }
    println!("upvalues ({})",p.upval.len());
    for (i,x) in p.upval.iter().enumerate() {
        println!("\t{}\t{}\t{}\t{}",i+1,p.upvalueNames[i],x.Instack,x.Idx);
    }
}

fn printCode(p: &ProtoType) {
    for (c, x) in p.code.iter().enumerate() {
        let mut line = String::new();
        if !p.lineInfo.is_empty() {
            line = format!("{}",p.lineInfo[c]);
        }
        println!("\t{}\t[{}]\t0x{:08X}",c+1,line,x);
    }
}

fn printHeader(p: &ProtoType) {
    let mut name = "main";
    if p.lineDefined>0 {
        name = "function";
    }
    let mut varArgFlag = "";
    if p.isVarArg>0 {
        varArgFlag = "+";
    }
    println!("\n{} <{}:{},{}> ({} instructions)",name,p.source,p.lineDefined,p.lastLineDefined,p.code.len());
    println!("{}{} params, {} slots, {} upvalues, {} locals,{} constants, {} functions",
             p.numParams,varArgFlag,p.StackSize,p.upval.len(),p.locVars.len(),p.constants.len(),p.protos.len());

}
