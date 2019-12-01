pub const MASS_LIST: &'static str = include_str!("mass-list.txt");

fn fuel_for_mass(mass: isize) -> isize {

    let fuel_needed: isize = mass / 3 - 2;

    if fuel_needed > 0 {
        fuel_needed + fuel_for_mass(fuel_needed)
    } else {
        0
    }
}

pub fn part1() {

    let mut fuel_needed = 0;

    for line in MASS_LIST.lines() {
        let mass: usize = line.parse().unwrap();
        fuel_needed += mass / 3 - 2;
    }

    println!("{}", fuel_needed);
}

pub fn part2() {

    let mut fuel_needed = 0;

    for line in MASS_LIST.lines() {
        let mass: isize = line.parse().unwrap();
        fuel_needed += fuel_for_mass(mass);
    }

    println!("{}", fuel_needed);
}
