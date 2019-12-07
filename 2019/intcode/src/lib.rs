//! Implementation of the intcode computer for AoC 2019

use std::convert::TryInto;
use std::io;
use std::num::ParseIntError;


/// Error encountered during the execution of an intcode program
#[derive(Debug)]
pub enum Error {

    /// Invalid address
    Address,

    /// Unrecognized opcode
    Opcode,

    /// Requested operation is not valid in the current state
    State,
}


/// Defines how to handle I/O operations
pub trait IoHandler {

    /// Retrieves a single integer as input
    fn input(&mut self) -> isize;

    /// Outputs a single integer
    fn output(&mut self, val: isize);
}


/// Default I/O handler
pub struct DefaultIoHandler;

impl IoHandler for DefaultIoHandler {

    fn input(&mut self) -> isize {

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("failed to read input");

        input.trim()
            .parse()
            .expect("failed to parse input")
    }

    fn output(&mut self, val: isize) {

        println!("{}", val);
    }
}


/// Tracks state of an executing CPU
struct Cpu<'a, H> {

    /// Instruction pointer
    ip: usize,

    /// Main memory
    mem: &'a mut [isize],

    /// I/O handler
    io: &'a mut H,
}

impl<'a, H> Cpu<'a, H>
where H: IoHandler
{

    fn decode_op(&self) -> isize {

        let op = self.mem[self.ip];
        let op = format!("{:0>2}", op);
        let op_idx = op.len() - 2;

        op[op_idx..].parse()
            .expect("failed to parse opcode")
    }

    fn load_param(&self, param_idx: usize) -> Result<isize, Error> {

        let is_immediate = {
            let op = format!("{:0>10}", self.mem[self.ip]);
            let flag_idx = op.len() - 3 - param_idx;
            &op[flag_idx..(flag_idx + 1)] == "1"
        };

        let param_addr: usize = if is_immediate {
            self.ip + 1 + param_idx
        } else {
            self.mem[self.ip + 1 + param_idx]
                .try_into()
                .map_err(|_| Error::Address)?
        };

        Ok(self.mem[param_addr])
    }

    fn store_by_param(
        &mut self,
        param_idx: usize,
        val: isize,
    ) -> Result<(), Error> {

        let dest_addr: usize = self.mem[self.ip + 1 + param_idx]
            .try_into()
            .map_err(|_| Error::Address)?;

        self.mem[dest_addr] = val;

        Ok(())
    }

    fn add(&mut self) -> Result<(), Error> {

        let lhs = self.load_param(0)?;
        let rhs = self.load_param(1)?;

        self.store_by_param(2, lhs + rhs)?;

        self.ip += 4;

        Ok(())
    }

    fn mul(&mut self) -> Result<(), Error> {

        let lhs = self.load_param(0)?;
        let rhs = self.load_param(1)?;

        self.store_by_param(2, lhs * rhs)?;

        self.ip += 4;

        Ok(())
    }

    fn input(&mut self) -> Result<(), Error> {

        let val = self.io.input();
        self.store_by_param(0, val)?;

        self.ip += 2;

        Ok(())
    }

    fn output(&mut self) -> Result<(), Error> {

        let val = self.load_param(0)?;
        self.io.output(val);

        self.ip += 2;

        Ok(())
    }

    fn jump_if_true(&mut self) -> Result<(), Error> {

        let val = self.load_param(0)?;

        if val != 0 {
            self.ip = self.load_param(1)?
                .try_into()
                .map_err(|_| Error::Address)?;
        } else {
            self.ip += 3;
        }

        Ok(())
    }

    fn jump_if_false(&mut self) -> Result<(), Error> {

        let val = self.load_param(0)?;

        if val == 0 {
            self.ip = self.load_param(1)?
                .try_into()
                .map_err(|_| Error::Address)?;
        } else {
            self.ip += 3;
        }

        Ok(())
    }

    fn less_than(&mut self) -> Result<(), Error> {

        let lhs = self.load_param(0)?;
        let rhs = self.load_param(1)?;

        self.store_by_param(2, if lhs < rhs { 1 } else { 0 })?;

        self.ip += 4;

        Ok(())
    }

    fn equals(&mut self) -> Result<(), Error> {

        let lhs = self.load_param(0)?;
        let rhs = self.load_param(1)?;

        self.store_by_param(2, if lhs == rhs { 1 } else { 0 })?;

        self.ip += 4;

        Ok(())
    }

    fn cycle(&mut self) -> Result<bool, Error> {

        match self.decode_op() {
            1  => self.add()?,
            2  => self.mul()?,
            3  => self.input()?,
            4  => self.output()?,
            5  => self.jump_if_true()?,
            6  => self.jump_if_false()?,
            7  => self.less_than()?,
            8  => self.equals()?,
            99 => return Ok(false),
            _  => return Err(Error::Opcode),
        }

        Ok(true)
    }
}


/// A computer capable of executing Intcode programs
pub struct Computer<H> {

    /// I/O handler used by this computer
    io: H,
}

impl<H> Computer<H>
where H: IoHandler
{

    pub fn new(io: H) -> Self {
        Self {
            io,
        }
    }

    pub fn eval(&mut self, mem: &mut [isize]) -> Result<(), Error> {

        let mut cpu = Cpu { ip: 0, mem, io: &mut self.io };

        while cpu.cycle()? { }

        Ok(())
    }
}


/// Parses a textual representation of an intcode program
pub fn parse_prog(prog: &str) -> Result<Vec<isize>, ParseIntError> {

    prog.split(",")
        .map(|i| i.trim().parse())
        .collect()
}


pub fn eval(prog: &mut [isize]) -> Result<(), Error> {

    Computer::new(DefaultIoHandler)
        .eval(prog)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day2_part1_case1() {

        let mut prog = [
            1,9,10,3,
            2,3,11,0,
            99,
            30,40,50,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            3500,9,10,70,
            2,3,11,0,
            99,
            30,40,50
        ]);
    }

    #[test]
    fn day2_part1_case2() {

        let mut prog = [
            1,0,0,0,
            99,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            2,0,0,0,
            99,
        ]);
    }

    #[test]
    fn day2_part1_case3() {

        let mut prog = [
            2,3,0,3,
            99,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            2,3,0,6,
            99,
        ]);
    }

    #[test]
    fn day2_part1_case4() {

        let mut prog = [
            2,4,4,5,
            99,
            0,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            2,4,4,5,
            99,
            9801,
        ]);
    }

    #[test]
    fn day2_part1_case5() {

        let mut prog = [
            1,1,1,4,
            99,
            5,6,0,99,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            30,1,1,4,
            2,5,6,0,
            99,
        ]);
    }

    const GRAV_PROG: &'static str = include_str!("grav-prog.txt");

    #[test]
    fn day2_part1_sln() {

        let mut prog = parse_prog(GRAV_PROG)
            .unwrap();

        // Restore to state just before spontaneous combustion
        prog[1] = 12;
        prog[2] = 2;

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog[0], 10566835);
    }

    #[test]
    fn day2_part2_sln() {

        let prog = parse_prog(GRAV_PROG)
            .unwrap();

        let mut res = None;

        'outer: for i in 0..100 {
            for j in 0..100 {

                let mut prog = prog.clone();

                // Set noun and verb
                prog[1] = i;
                prog[2] = j;

                eval(&mut prog)
                    .unwrap();

                if prog[0] == 19690720 {
                    res = Some(100 * i + j);
                    break 'outer;
                }
            }
        }

        assert_eq!(res, Some(2347));
    }

    #[test]
    fn day5_part1_case1() {

        // Custom handler to test simple I/O
        struct SimpleHandler<'a> {
            input: isize,
            output: &'a mut Vec<isize>,
        }

        impl<'a> IoHandler for SimpleHandler<'a> {
            fn input(&mut self) -> isize {
                self.input
            }
            fn output(&mut self, val: isize) {
                self.output.push(val);
            }
        }

        let prog = vec![
            3,0,
            4,0,
            99,
        ];

        for i in 0..100 {

            let mut output = vec![];
            let io = SimpleHandler { input: i, output: &mut output };

            let mut prog = prog.clone();

            Computer::new(io)
                .eval(&mut prog)
                .unwrap();

            assert_eq!(output, vec![i]);
        }
    }

    #[test]
    fn day5_part1_case2() {

        let mut prog = [
            1002,4,3,4,
            33,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            1002,4,3,4,
            99,
        ]);
    }

    #[test]
    fn day5_part1_case3() {

        let mut prog = [
            1101,100,-1,4,
            0,
        ];

        eval(&mut prog)
            .unwrap();

        assert_eq!(prog, [
            1101,100,-1,4,
            99,
        ]);
    }

    // TODO: port remaining day5 unit tests

    // #[test]
    // fn day5_part2_case1() {

    //     let mut prog = [
    //         3,9,8,9,10,9,4,9,99,-1,8
    //     ];
    // }
}
