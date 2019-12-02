enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn eval(&self, prog: &mut [usize], i: usize) {

        let lidx = prog[i + 1];
        let lhs = prog[lidx];

        let ridx = prog[i + 2];
        let rhs = prog[ridx];

        let dest = prog[i + 3];
        prog[dest] = match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        };
    }
}

fn eval(prog: &mut [usize]) {

    let mut i = 0;
    loop {

        let op = prog[i];
        let op = match op {
            1 => Operation::Add,
            2 => Operation::Multiply,
            99 => return,
            _ => panic!("unknown opcode {}", op),
        };

        op.eval(prog, i);
        i += 4;
    }
}

pub fn part1() {

    let mut prog = [
        1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,6,23,27,1,5,27,31,2,31,9,35,1,35,5,39,1,39,5,43,1,43,10,47,2,6,47,51,1,51,5,55,2,55,6,59,1,5,59,63,2,63,6,67,1,5,67,71,1,71,6,75,2,75,10,79,1,79,5,83,2,83,6,87,1,87,5,91,2,9,91,95,1,95,6,99,2,9,99,103,2,9,103,107,1,5,107,111,1,111,5,115,1,115,13,119,1,13,119,123,2,6,123,127,1,5,127,131,1,9,131,135,1,135,9,139,2,139,6,143,1,143,5,147,2,147,6,151,1,5,151,155,2,6,155,159,1,159,2,163,1,9,163,0,99,2,0,14,0
    ];

    // Restore to state just before spontaneous combustion
    prog[1] = 12;
    prog[2] = 2;

    eval(&mut prog);

    println!("{}", prog[0]);
}

pub fn part2() {

    for i in 0..100 {
        for j in 0..100 {
            let mut prog = [
                1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,6,23,27,1,5,27,31,2,31,9,35,1,35,5,39,1,39,5,43,1,43,10,47,2,6,47,51,1,51,5,55,2,55,6,59,1,5,59,63,2,63,6,67,1,5,67,71,1,71,6,75,2,75,10,79,1,79,5,83,2,83,6,87,1,87,5,91,2,9,91,95,1,95,6,99,2,9,99,103,2,9,103,107,1,5,107,111,1,111,5,115,1,115,13,119,1,13,119,123,2,6,123,127,1,5,127,131,1,9,131,135,1,135,9,139,2,139,6,143,1,143,5,147,2,147,6,151,1,5,151,155,2,6,155,159,1,159,2,163,1,9,163,0,99,2,0,14,0
            ];

            // Set noun and verb
            prog[1] = i;
            prog[2] = j;

            eval(&mut prog);

            if prog[0] == 19690720 {
                println!("{}", 100 * i + j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_case0() {

        let mut prog = [
            1,9,10,3,
            2,3,11,0,
            99,
            30,40,50
        ];

        eval(&mut prog);

        assert_eq!(prog, [
            3500,9,10,70,
            2,3,11,0,
            99,
            30,40,50
        ]);
    }

    #[test]
    fn part1_case1() {

        let mut prog = [1,0,0,0,99];

        eval(&mut prog);

        assert_eq!(prog, [2,0,0,0,99]);
    }

    #[test]
    fn part1_case2() {

        let mut prog = [2,3,0,3,99];

        eval(&mut prog);

        assert_eq!(prog, [2,3,0,6,99]);
    }

    #[test]
    fn part1_case3() {

        let mut prog = [2,4,4,5,99,0];

        eval(&mut prog);

        assert_eq!(prog, [2,4,4,5,99,9801]);
    }

    #[test]
    fn part1_case4() {

        let mut prog = [1,1,1,4,99,5,6,0,99];

        eval(&mut prog);

        assert_eq!(prog, [30,1,1,4,2,5,6,0,99]);
    }
}
