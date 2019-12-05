use std::io::{self, BufRead};
use std::convert::TryFrom;

enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
}

fn param_is_immediate(op: isize, param: usize) -> bool {

    let op = format!("{:0>10}", op); // limits opcodes to 10 digits

    let flag_idx = op.len() - 3 - param;

    &op[flag_idx..(flag_idx+1)] == "1"
}

// TODO: use this everywhere
fn load_param(prog: &[isize], ip: usize, param: usize) -> isize {

    let op = prog[ip];

    if param_is_immediate(op, param) {
        prog[ip + 1 + param]
    } else {
        let addr = usize::try_from(prog[ip + 1 + param])
            .unwrap();
        prog[addr]
    }
}

impl Operation {

    fn eval(&self, prog: &mut [isize], ip: usize) -> Option<usize> {

        let op = prog[ip];

        match self {

            Self::Add => {

                let lhs = load_param(prog, ip, 0);

                let rhs = load_param(prog, ip, 1);

                let dest = usize::try_from(prog[ip + 3]).unwrap();
                prog[dest] = lhs + rhs;
            },

            Self::Multiply => {

                let lhs = if param_is_immediate(op, 0) {
                    prog[ip + 1]
                } else {
                    let addr = usize::try_from(prog[ip + 1]).unwrap();
                    prog[addr]
                };

                let rhs = if param_is_immediate(op, 1) {
                    prog[ip + 2]
                } else {
                    let addr = usize::try_from(prog[ip + 2]).unwrap();
                    prog[addr]
                };

                let dest = usize::try_from(prog[ip + 3]).unwrap();
                prog[dest] = lhs * rhs;
            },

            Self::Input => {

                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .unwrap();
                let val: isize = input.trim().parse()
                    .unwrap();

                let dest = usize::try_from(prog[ip + 1]).unwrap();
                prog[dest] = val;
            },

            Self::Output => {

                let val = if param_is_immediate(op, 0) {
                    prog[ip + 1]
                } else {
                    let addr = usize::try_from(prog[ip + 1]).unwrap();
                    prog[addr]
                };

                println!("{}", val);
            },

            Self::JumpTrue => {

                let val = load_param(prog, ip, 0);

                if val != 0 {
                    let dest = usize::try_from(load_param(prog, ip, 1))
                        .unwrap();
                    return Some(dest);
                }
            },

            Self::JumpFalse => {

                let val = load_param(prog, ip, 0);

                if val == 0 {
                    let dest = usize::try_from(load_param(prog, ip, 1))
                        .unwrap();
                    return Some(dest);
                }
            },

            Self::Less => {

                let lhs = load_param(prog, ip, 0);
                let rhs = load_param(prog, ip, 1);

                let dest = usize::try_from(prog[ip + 3])
                    .unwrap();
                prog[dest] = if lhs < rhs { 1 } else { 0 };
            },

            Self::Equal => {

                let lhs = load_param(prog, ip, 0);
                let rhs = load_param(prog, ip, 1);

                let dest = usize::try_from(prog[ip + 3])
                    .unwrap();
                prog[dest] = if lhs == rhs { 1 } else { 0 };
            },
        }

        None
    }

    fn size(&self) -> usize {

        match self {
            Self::Add => 4,
            Self::Multiply => 4,
            Self::Input => 2,
            Self::Output => 2,
            Self::JumpTrue => 3,
            Self::JumpFalse => 3,
            Self::Less => 4,
            Self::Equal => 4,
        }
    }
}

fn parse_op(op: isize) -> isize {

    let op = format!("{:0>2}", op);
    op[(op.len() - 2)..].parse().unwrap()
}

fn eval(prog: &mut [isize]) {

    let mut i = 0;
    loop {

        let op = parse_op(prog[i]);
        let op = match op {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpTrue,
            6 => Operation::JumpFalse,
            7 => Operation::Less,
            8 => Operation::Equal,
            99 => return,
            _ => panic!("unknown opcode {}", op),
        };

        if let Some(new_ip) = op.eval(prog, i) {
            i = new_ip;
        } else {
            i += op.size();
        }
    }
}

pub fn part1() {

    let mut prog = [
        3,225,1,225,6,6,1100,1,238,225,104,0,1102,17,65,225,102,21,95,224,1001,224,-1869,224,4,224,1002,223,8,223,101,7,224,224,1,224,223,223,101,43,14,224,1001,224,-108,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,57,94,225,1101,57,67,225,1,217,66,224,101,-141,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1102,64,34,225,1101,89,59,225,1102,58,94,225,1002,125,27,224,101,-2106,224,224,4,224,102,8,223,223,1001,224,5,224,1,224,223,223,1102,78,65,225,1001,91,63,224,101,-127,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1102,7,19,224,1001,224,-133,224,4,224,102,8,223,223,101,6,224,224,1,224,223,223,2,61,100,224,101,-5358,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,19,55,224,101,-74,224,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,1101,74,68,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,344,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,359,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,374,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,389,101,1,223,223,8,677,226,224,1002,223,2,223,1005,224,404,101,1,223,223,1108,677,226,224,102,2,223,223,1006,224,419,1001,223,1,223,1108,226,677,224,102,2,223,223,1006,224,434,101,1,223,223,1108,677,677,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,677,677,224,1002,223,2,223,1006,224,464,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,479,101,1,223,223,108,677,677,224,1002,223,2,223,1005,224,494,101,1,223,223,107,226,677,224,1002,223,2,223,1006,224,509,101,1,223,223,107,226,226,224,102,2,223,223,1006,224,524,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,539,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,554,1001,223,1,223,8,226,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1007,677,677,224,102,2,223,223,1005,224,584,1001,223,1,223,1107,677,226,224,1002,223,2,223,1006,224,599,101,1,223,223,7,226,226,224,1002,223,2,223,1005,224,614,101,1,223,223,108,677,226,224,1002,223,2,223,1005,224,629,1001,223,1,223,108,226,226,224,1002,223,2,223,1005,224,644,101,1,223,223,1007,677,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,226,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226

    ];

    eval(&mut prog);

    // println!("{}", param_is_immediate(1001, 1));
}

pub fn part2() {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_case0() {
        unimplemented!();
    }

    #[test]
    fn part2_case0() {
        unimplemented!();
    }
}
