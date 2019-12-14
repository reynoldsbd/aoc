
use std::collections::HashMap;
use std::str::FromStr;


#[derive(Debug)]
struct Reactions(HashMap<String, (HashMap<String, u64>, u64)>);

impl FromStr for Reactions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut reactions = HashMap::new();

        for s in s.lines() {

            let mut sides = s.split("=>");
            let lhs = sides.next().unwrap();
            let rhs = sides.next().unwrap();
            assert!(sides.next().is_none());

            let mut inputs = HashMap::new();
            for input in lhs.split(",") {
                let mut parts = input.trim()
                    .split(" ")
                    .map(|p| p.trim());
                let count = parts.next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let chem = parts.next()
                    .unwrap()
                    .to_string();
                assert!(parts.next().is_none());
                inputs.insert(chem, count);
            }

            let mut parts = rhs.trim()
                .split(" ")
                .map(|p| p.trim());
            let output_count = parts.next()
                .unwrap()
                .parse()
                .unwrap();
            let output_chem = parts.next()
                .unwrap()
                .to_string();
            assert!(parts.next().is_none());

            reactions.insert(output_chem, (inputs, output_count));
        }

        Ok(Self(reactions))
    }
}


#[derive(Debug)]
struct Nanofactory {
    ore_avail: u64,
    chems_available: HashMap<String, u64>,
}

impl Nanofactory {

    fn produce(&mut self, chem: &str, reactions: &Reactions) -> bool {

        let (chems_needed, amount_produced) = &reactions.0[chem];

        for (chem, amount_needed) in chems_needed {

            if chem == "ORE" {

                if *amount_needed > self.ore_avail {
                    return false;
                } else {
                    self.ore_avail -= amount_needed;
                }

            } else {

                if !self.chems_available.contains_key(chem) {
                    self.chems_available.insert(chem.clone(), 0);
                }

                while self.chems_available[chem] < *amount_needed {
                    if !self.produce(chem, reactions) {
                        return false;
                    }
                }

                *self.chems_available.get_mut(chem).unwrap() -= amount_needed;
            }

        }

        if !self.chems_available.contains_key(chem) {
            self.chems_available.insert(chem.into(), 0);
        }

        *self.chems_available.get_mut(chem).unwrap() += amount_produced;

        true
    }
}


const REACTIONS: &'static str = include_str!("reactions.txt");


pub fn part1() {

    let reactions: Reactions = REACTIONS.parse()
        .unwrap();

    let total_ore = 1_000_000_000;

    let mut nf = Nanofactory {
        ore_avail: total_ore,
        chems_available: HashMap::new(),
    };

    assert!(nf.produce("FUEL", &reactions));

    let ore_used = total_ore - nf.ore_avail;

    println!("{}", ore_used);
}

pub fn part2() {

    let reactions: Reactions = REACTIONS.parse()
        .unwrap();

    let ore_avail = 1_000_000_000_000;

    let mut nf = Nanofactory {
        ore_avail: ore_avail,
        chems_available: HashMap::new(),
    };

    while nf.produce("FUEL", &reactions) {}

    println!("{}", nf.chems_available["FUEL"]);
}
