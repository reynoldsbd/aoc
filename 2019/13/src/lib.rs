
use std::collections::HashMap;
use std::io;

use intcode::{Computer, IoHandler};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

enum GameOutputState {
    XCoord,
    YCoord {
        x: isize,
    },
    TileId {
        loc: Point,
    },
}

struct GameHandler {
    state: GameOutputState,
    tiles: HashMap<Point, isize>,
    ball_cur: Option<Point>,
    ball_prev: Option<Point>,
    paddle_cur: Option<Point>,
}

impl GameHandler {

    fn render(&self) {

        let max_x = self.tiles.keys()
            .map(|k| k.x)
            .max()
            .unwrap();
        let max_y = self.tiles.keys()
            .map(|k| k.y)
            .max()
            .unwrap();

        for y in 0..=max_y {

            for x in 0..=max_x {

                let p = Point { x, y };

                match self.tiles.get(&p) {

                    // Empty space
                    None | Some(0) => print!(" "),

                    // Wall
                    Some(1) => print!("#"),

                    // Block
                    Some(2) => print!("="),

                    // Paddle
                    Some(3) => print!("_"),

                    // Ball
                    Some(4) => print!("*"),

                    Some(id) => panic!("unexpected tile id {}", id),
                }
            }

            print!("\n");
        }
    }
}

impl IoHandler for &mut GameHandler {

    fn input(&mut self) -> isize {

        self.render();

        let ball_x = self.ball_cur.unwrap().x;
        let ball_dx = if let Some(ball_prev) = self.ball_prev {
            ball_x - ball_prev.x
        } else {
            0
        };
        let ball_x_next = ball_x + ball_dx;

        let paddle_x = self.paddle_cur.unwrap().x;
        let prediction = if paddle_x < ball_x_next {
            1
        } else if paddle_x > ball_x_next {
            -1
        } else {
            0
        };

        println!("paddle_x:    {}", paddle_x);
        println!("ball_x_next: {}", ball_x_next);
        println!("prediction:  {}", prediction);

        loop {
            println!("next move? (p/l/r/n)");
            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .unwrap();

            match line.trim() {
                "p" | "" => return prediction,
                "n" => return 0,
                "l" => return -1,
                "r" => return 1,
                _ => println!("unrecognized input!"),
            }
        }
    }

    fn output(&mut self, val: isize) {

        match &self.state {

            GameOutputState::XCoord => {
                self.state = GameOutputState::YCoord {
                    x: val,
                };
            },

            GameOutputState::YCoord { x } => {
                self.state = GameOutputState::TileId {
                    loc: Point {
                        x: *x,
                        y: val,
                    },
                };
            },

            GameOutputState::TileId { loc } => {

                let loc = *loc;

                if loc.x == -1 && loc.y == 0 {
                    println!("Score: {}", val);
                } else {
                    self.tiles.insert(loc, val);
                    if val == 3 {
                        self.paddle_cur = Some(loc);
                    } else if val == 4 {
                        if self.ball_cur.is_some() {
                            self.ball_prev = self.ball_cur;
                        }
                        self.ball_cur = Some(loc);
                    }
                }

                self.state = GameOutputState::XCoord;
            },
        }
    }
}

const GAME: &'static str = include_str!("game.txt");

pub fn part1() {

    let mut game = intcode::parse_prog(GAME)
        .unwrap();
    let mut handler = GameHandler {
        state: GameOutputState::XCoord,
        tiles: HashMap::new(),
        ball_cur: None,
        ball_prev: None,
        paddle_cur: None,
    };

    Computer::new(&mut handler)
        .eval(&mut game)
        .unwrap();

    let mut blocks = 0;
    for (_, id) in handler.tiles {
        if id == 2 {
            blocks += 1;
        }
    }

    println!("{}", blocks);
}

pub fn part2() {

    let mut game = intcode::parse_prog(GAME)
        .unwrap();
    let mut handler = GameHandler {
        state: GameOutputState::XCoord,
        tiles: HashMap::new(),
        ball_cur: None,
        ball_prev: None,
        paddle_cur: None,
    };

    // Jimmy the quarter slot
    game[0] = 2;

    Computer::new(&mut handler)
        .eval(&mut game)
        .unwrap();
}
