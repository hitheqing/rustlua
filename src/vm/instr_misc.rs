use crate::api::consts::*;
use crate::api::{LuaAPI, LuaVM};
use crate::state::LuaState;
use crate::vm::instruction::Instruction;

pub fn _move(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, _c) = i.abc();
    a += 1;
    b += 1;
    vm.copy(b, a);
}

pub fn jmp(i: u32, vm: &mut LuaState) {
    let (a, sbx) = i.a_sbx();

    vm.add_pc(sbx);
    if a != 0 {
        panic!("todo: jmp!");
    }
}

//--load-----------------------------------------------------------------------------------------------

// R(A), R(A+1), ..., R(A+B) := nil
pub fn load_nil(i: u32, vm: &mut LuaState) {
    let (mut a, b, _) = i.abc();
    a += 1;

    vm.push_nil();
    for i in a..(a + b + 1) {
        vm.copy(-1, i);
    }
    vm.pop(1);
}

// R(A) := (bool)B; if (C) pc++
pub fn load_bool(i: u32, vm: &mut LuaState) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.push_boolean(b != 0);
    vm.replace(a);

    if c != 0 {
        vm.add_pc(1);
    }
}

// R(A) := Kst(Bx)
pub fn load_k(i: u32, vm: &mut LuaState) {
    let (mut a, bx) = i.a_bx();
    a += 1;

    vm.get_const(bx);
    vm.replace(a);
}

// R(A) := Kst(extra arg)
pub fn load_kx(i: u32, vm: &mut LuaState) {
    let (mut a, _) = i.a_bx();
    a += 1;
    let ax = vm.fetch().ax();

    //vm.CheckStack(1)
    vm.get_const(ax);
    vm.replace(a);
}

//--load end-----------------------------------------------------------------------------------------------

//--ops-----------------------------------------------------------------------------------------------

/* arith */
pub fn add(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPADD)
} // +
pub fn sub(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPSUB)
} // -
pub fn mul(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPMUL)
} // *
pub fn _mod(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPMOD)
} // %
pub fn pow(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPPOW)
} // ^
pub fn div(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPDIV)
} // /
pub fn idiv(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPIDIV)
} // //
pub fn band(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPBAND)
} // &
pub fn bor(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPBOR)
} // |
pub fn bxor(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPBXOR)
} // ~
pub fn shl(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPSHL)
} // <<
pub fn shr(i: u32, vm: &mut LuaState) {
    binary_arith(i, vm, LUA_OPSHR)
} // >>
pub fn unm(i: u32, vm: &mut LuaState) {
    unary_arith(i, vm, LUA_OPUNM)
} // -
pub fn bnot(i: u32, vm: &mut LuaState) {
    unary_arith(i, vm, LUA_OPBNOT)
} // ~

// R(A) := RK(B) op RK(C)
fn binary_arith(i: u32, vm: &mut LuaState, op: u8) {
    let (mut a, b, c) = i.abc();
    a += 1;

    vm.get_rk(b);
    vm.get_rk(c);
    vm.arith(op);
    vm.replace(a);
}

// R(A) := op R(B)
fn unary_arith(i: u32, vm: &mut LuaState, op: u8) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.push_value(b);
    vm.arith(op);
    vm.replace(a);
}

/* compare */

pub fn eq(i: u32, vm: &mut LuaState) {
    compare(i, vm, LUA_OPEQ)
} // ==
pub fn lt(i: u32, vm: &mut LuaState) {
    compare(i, vm, LUA_OPLT)
} // <
pub fn le(i: u32, vm: &mut LuaState) {
    compare(i, vm, LUA_OPLE)
} // <=

// if ((RK(B) op RK(C)) ~= A) then pc++
fn compare(i: u32, vm: &mut LuaState, op: u8) {
    let (a, b, c) = i.abc();

    vm.get_rk(b);
    vm.get_rk(c);
    if vm.compare(-2, -1, op) != (a != 0) {
        vm.add_pc(1);
    }
    vm.pop(2);
}

/* logical */

// R(A) := not R(B)
pub fn not(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.push_boolean(!vm.to_boolean(b));
    vm.replace(a);
}

// if not (R(A) <=> C) then pc++
pub fn test(i: u32, vm: &mut LuaState) {
    let (mut a, _, c) = i.abc();
    a += 1;

    if vm.to_boolean(a) != (c != 0) {
        vm.add_pc(1);
    }
}

// if (R(B) <=> C) then R(A) := R(B) else pc++
pub fn test_set(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, c) = i.abc();
    a += 1;
    b += 1;

    if vm.to_boolean(b) == (c != 0) {
        vm.copy(b, a);
    } else {
        vm.add_pc(1);
    }
}

/* len & concat */

// R(A) := length of R(B)
pub fn length(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, _) = i.abc();
    a += 1;
    b += 1;

    vm.len(b);
    vm.replace(a);
}

// R(A) := R(B).. ... ..R(C)
pub fn concat(i: u32, vm: &mut LuaState) {
    let (mut a, mut b, mut c) = i.abc();
    a += 1;
    b += 1;
    c += 1;

    let n = c - b + 1;
    vm.check_stack(n as usize);
    for i in b..(c + 1) {
        vm.push_value(i);
    }
    vm.concat(n);
    vm.replace(a);
}

//--ops end-----------------------------------------------------------------------------------------------

//--for-----------------------------------------------------------------------------------------------

// R(A)-=R(A+2); pc+=sBx
pub fn for_prep(i: u32, vm: &mut LuaState) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    if vm.type_id(a) == LUA_TSTRING {
        vm.push_number(vm.to_number(a));
        vm.replace(a);
    }
    if vm.type_id(a + 1) == LUA_TSTRING {
        vm.push_number(vm.to_number(a + 1));
        vm.replace(a + 1);
    }
    if vm.type_id(a + 2) == LUA_TSTRING {
        vm.push_number(vm.to_number(a + 2));
        vm.replace(a + 2);
    }

    vm.push_value(a);
    vm.push_value(a + 2);
    vm.arith(LUA_OPSUB);
    vm.replace(a);
    vm.add_pc(sbx);
}

// R(A)+=R(A+2);
// if R(A) <?= R(A+1) then {
//   pc+=sBx; R(A+3)=R(A)
// }
pub fn for_loop(i: u32, vm: &mut LuaState) {
    let (mut a, sbx) = i.a_sbx();
    a += 1;

    // R(A)+=R(A+2);
    vm.push_value(a + 2);
    vm.push_value(a);
    vm.arith(LUA_OPADD);
    vm.replace(a);

    let positive_step = vm.to_number(a + 2) >= 0.0;
    if positive_step && vm.compare(a, a + 1, LUA_OPLE) || !positive_step && vm.compare(a + 1, a, LUA_OPLE) {
        // pc+=sBx; R(A+3)=R(A)
        vm.add_pc(sbx);
        vm.copy(a, a + 3);
    }
}
//--for end-----------------------------------------------------------------------------------------------