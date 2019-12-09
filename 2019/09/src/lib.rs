const BOOST: &'static str = include_str!("boost.txt");

pub fn part1() {
    let mut prog = intcode::parse_prog(BOOST)
        .unwrap();

    intcode::eval(&mut prog)
        .unwrap();
}

pub fn part2() {
    unimplemented!();
}
