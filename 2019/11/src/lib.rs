use std::collections::HashMap;

use intcode::{Computer, IoHandler};


#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

enum HullBotState {
    Painting,
    Turning,
}

struct HullBot {
    loc: Point,
    dx: isize,
    dy: isize,
    state: HullBotState,
    hull: HashMap<Point, isize>,
}

impl HullBot {

    fn new() -> Self {
        Self {
            loc: Point { x: 0, y: 0 },
            dx: 0,
            dy: 1,
            state: HullBotState::Painting,
            hull: HashMap::new(),
        }
    }
}

impl IoHandler for &mut HullBot {

    fn input(&mut self) -> isize {

        *self.hull.get(&self.loc)
            .unwrap_or(&0)
    }

    fn output(&mut self, val: isize) {

        match self.state {

            HullBotState::Painting => {

                self.hull.insert(self.loc, val);

                self.state = HullBotState::Turning;
            },

            HullBotState::Turning => {

                let dx = self.dx;
                let dy = self.dy;

                match val {
                    0 => {
                        self.dx = -1 * dy;
                        self.dy = dx;
                    },
                    1 => {
                        self.dx = dy;
                        self.dy = -1 * dx;
                    },
                    _ => panic!("unexpected rotation direction {}", val),
                }

                self.loc = Point {
                    x: self.loc.x + self.dx,
                    y: self.loc.y + self.dy,
                };

                self.state = HullBotState::Painting;
            },
        }
    }
}

const HULL_PAINT: &'static str = include_str!("hull-paint.txt");

pub fn part1() {

    let mut hull_paint = intcode::parse_prog(HULL_PAINT)
        .unwrap();

    let mut hull_bot = HullBot::new();

    Computer::new(&mut hull_bot)
        .eval(&mut hull_paint)
        .unwrap();

    println!("{}", hull_bot.hull.len());
}

pub fn part2() {

    let mut hull_paint = intcode::parse_prog(HULL_PAINT)
        .unwrap();

    let mut hull_bot = HullBot::new();

    // Start on a white panel
    hull_bot.hull.insert(Point { x: 0, y: 0 }, 1);

    Computer::new(&mut hull_bot)
        .eval(&mut hull_paint)
        .unwrap();

    let min_x = hull_bot.hull.keys()
        .map(|p| p.x)
        .min()
        .unwrap();
    let max_x = hull_bot.hull.keys()
        .map(|p| p.x)
        .max()
        .unwrap();
    let min_y = hull_bot.hull.keys()
        .map(|p| p.y)
        .min()
        .unwrap();
    let max_y = hull_bot.hull.keys()
        .map(|p| p.y)
        .max()
        .unwrap();

    println!("x: {}..={}", min_x, max_x);
    println!("y: {}..={}", min_y, max_y);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            match hull_bot.hull.get(&Point { x, y }) {
                Some(1) => print!("#"),
                _       => print!("."),
            }
        }
        print!("\n");
    }
}
