use std::num::ParseIntError;


/// Error encountered during the execution of an intcode program
pub enum Error {

    /// Unrecognized opcode
    Opcode,

    /// Requested operation is not valid in the current state
    State,
}


fn decode_op(op: isize) -> isize {

    let op = format!("{:0>2}", op);
    let op_idx = op.len() - 2;

    op[op_idx..].parse()
        .expect("failed to parse opcode")
}


#[derive(PartialEq)]
enum CpuState {
    Idle,
    Running,
    Halted,
    Faulted,
}


/// A computer capable of executing Intcode programs
struct Cpu<'a> {

    /// Instruction pointer
    ip: usize,

    /// Main memory
    mem: &'a mut [isize],

    /// Current state of the CPU
    state: CpuState,
}

impl<'a> Cpu<'a> {

    /// Executes the current CPU instruction
    fn cycle(&mut self) -> Result<(), Error> {

        match self.state {
            CpuState::Idle => self.state = CpuState::Running,
            CpuState::Halted | CpuState::Faulted => return Err(Error::State),
            CpuState::Running => (),
        }

        match decode_op(self.mem[self.ip]) {
            _ => return Err(Error::Opcode),
        };

        Ok(())
    }
}


pub fn eval(mem: &mut [isize]) -> Result<(), Error> {

    let mut cpu = Cpu { ip: 0, mem, state: CpuState::Idle };

    while cpu.state != CpuState::Halted {
        cpu.cycle()?;
    }

    Ok(())
}
