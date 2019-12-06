//! Implementation of the intcode computer for AoC 2019

use std::convert::TryInto;
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


/// Tracks state of an executing CPU
struct Cpu<'a> {

    /// Instruction pointer
    ip: usize,

    /// Main memory
    mem: &'a mut [isize],
}

impl<'a> Cpu<'a> {

    fn decode_op(&self) -> isize {

        let op = self.mem[self.ip];
        let op = format!("{:0>2}", op);
        let op_idx = op.len() - 2;

        op[op_idx..].parse()
            .expect("failed to parse opcode")
    }

    fn load_param(&self, param_idx: usize) -> Result<isize, Error> {

        let param_addr: usize = self.mem[self.ip + 1 + param_idx]
            .try_into()
            .map_err(|_| Error::Address)?;

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

    fn cycle(&mut self) -> Result<bool, Error> {

        match self.decode_op() {
            1  => self.add()?,
            2  => self.mul()?,
            99 => return Ok(false),
            _  => return Err(Error::Opcode),
        }

        Ok(true)
    }
}


/// A computer capable of executing Intcode programs
pub struct Computer();

impl Computer {

    pub fn new() -> Self {
        Self()
    }

    pub fn eval(&self, mem: &mut [isize]) -> Result<(), Error> {

        let mut cpu = Cpu { ip: 0, mem };

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

        Computer::new()
            .eval(&mut prog)
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

        Computer::new()
            .eval(&mut prog)
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

        Computer::new()
            .eval(&mut prog)
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

        Computer::new()
            .eval(&mut prog)
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

        Computer::new()
            .eval(&mut prog)
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

        Computer::new()
            .eval(&mut prog)
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

                Computer::new()
                    .eval(&mut prog)
                    .unwrap();

                if prog[0] == 19690720 {
                    res = Some(100 * i + j);
                    break 'outer;
                }
            }
        }

        assert_eq!(res, Some(2347));
    }

    // TODO: port remaining unit tests
}
