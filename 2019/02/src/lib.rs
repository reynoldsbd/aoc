use intcode;


const GRAV_PROG: &'static str = include_str!("grav-prog.txt");


pub fn part1() {

    let mut prog = intcode::parse_prog(GRAV_PROG)
        .unwrap();

    // Restore to state just before spontaneous combustion
    prog[1] = 12;
    prog[2] = 2;

    intcode::eval(&mut prog)
        .unwrap();

    println!("{}", prog[0]);
}

pub fn part2() {

    'outer: for i in 0..100 {
        for j in 0..100 {

            let mut prog = intcode::parse_prog(GRAV_PROG)
                .unwrap();

            // Set noun and verb
            prog[1] = i;
            prog[2] = j;

            intcode::eval(&mut prog)
                .unwrap();

            if prog[0] == 19690720 {
                println!("{}", 100 * i + j);
                break 'outer;
            }
        }
    }
}
