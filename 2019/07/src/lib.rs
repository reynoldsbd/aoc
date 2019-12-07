use std::sync::mpsc::{self, *};
use std::thread;

use itertools::Itertools;

use intcode::{self, Computer, IoHandler};

const AMP_PROG: &'static str = include_str!("amp-prog.txt");


fn run_amp(phase: isize, input: isize) -> isize {

    struct AmpIo<'a> {
        phase: isize,
        input: isize,
        counter: isize,
        output: &'a mut Option<isize>,
    }
    impl<'a> IoHandler for AmpIo<'a> {
        fn input(&mut self) -> isize {
            match self.counter {
                0 => {
                    self.counter += 1;
                    self.phase
                },
                _ => self.input,
            }
        }
        fn output(&mut self, val: isize) {
            *self.output = Some(val);
        }
    }

    let mut output = None;
    let io = AmpIo {
        phase,
        input,
        counter: 0,
        output: &mut output,
    };

    let mut prog = intcode::parse_prog(AMP_PROG)
        .unwrap();
    Computer::new(io)
        .eval(&mut prog)
        .unwrap();

    output.unwrap()
}

fn run_amps() {

    let mut max: Option<isize> = None;

    for phases in (0..5).permutations(5) {
        let mut last_input = 0;
        for phase in phases {
            last_input = run_amp(phase, last_input);
        }

        if max.is_none() || max.unwrap() < last_input {
            max = Some(last_input);
        }
    }

    println!("{:?}", max);
}

fn run_amp_threaded(phase: isize, input: Receiver<isize>) -> Receiver<isize> {

    let (output, rx) = mpsc::channel();

    // TODO

    struct AmpIo {
        phase: isize,
        input: Receiver<isize>,
        counter: isize,
        output: Sender<isize>,
    }
    impl IoHandler for AmpIo {
        fn input(&mut self) -> isize {
            match self.counter {
                0 => {
                    self.counter += 1;
                    self.phase
                },
                _ => self.input.recv().unwrap(),
            }
        }
        fn output(&mut self, val: isize) {
            self.output.send(val).unwrap();
        }
    }

    let io = AmpIo {
        phase,
        input,
        counter: 0,
        output,
    };

    thread::spawn(move ||{
        let mut prog = intcode::parse_prog(AMP_PROG)
            .unwrap();
        Computer::new(io)
            .eval(&mut prog)
            .unwrap();
    });

    // TODO end

    rx
}

fn run_amps_feedback() {

    let mut max: Option<isize> = None;

    for phases in (5..10).permutations(5) {

        let (tx, mut rx) = mpsc::channel();

        for phase in phases {
            rx = run_amp_threaded(phase, rx);
        }

        tx.send(0)
            .unwrap();

        let mut last_out = None;
        while let Ok(out) = rx.recv() {
            last_out = Some(out);
            if tx.send(out).is_err() {
                break;
            }
        }

        if max.is_none() || max.unwrap() < last_out.unwrap() {
            max = last_out;
        }
    }

    println!("{:?}", max);
}


pub fn part1() {
    run_amps();
}

pub fn part2() {
    run_amps_feedback();
}
