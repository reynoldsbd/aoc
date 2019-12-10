use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {

    fn same_angle(&self, base: &Self, other: &Self) -> bool {

        let dx_self = self.x - base.x;
        let dy_self = self.y - base.y;
        let dx_other = other.x - base.x;
        let dy_other = other.y - base.y;

        let same_slope = dx_self * dy_other == dx_other * dy_self;

        let same_quad =
            dx_self.is_positive() == dx_other.is_positive()
            &&
            dy_self.is_positive() == dy_other.is_positive()
        ;

        same_slope && same_quad
    }

    fn dist(&self, base: &Self) -> f64 {

        let dx = (self.x - base.x) as f64;
        let dy = (self.y - base.y) as f64;

        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    fn angle_from(&self, base: &Self) -> f64 {

        let dx = (self.x - base.x) as f64;
        let dy = (self.y - base.y) as f64;

        let mut angle = dy.atan2(dx) * 180.0f64 / std::f64::consts::PI;

        // Adjust since laser starts at 90 degrees and moves clockwise
        angle -= 90.0;
        angle *= -1.0;
        if angle < 0.0 {
            angle += 360.0;
        }

        angle
    }
}

fn get_visible_counts(asteroids: &[Point]) -> Vec<(Point, usize)> {

    let mut counts = vec![];

    for this in asteroids {

        let mut visible = vec![];

        for other in asteroids {
            if !visible.iter().any(|a: &Point| a.same_angle(this, other)) {
                visible.push(*other);
            }
        }

        counts.push((*this, visible.len()));
    }

    counts
}


fn parse_map(map: &str) -> Vec<Point> {

    let mut points = vec![];

    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                let x: isize = x.try_into().unwrap();
                let y: isize = y.try_into().unwrap();
                let y = -1 * y;
                points.push(Point { x, y });
            }
        }
    }

    points
}

const MAP: &'static str = include_str!("map.txt");

pub fn part1() {

    let map = parse_map(MAP);

    let counts = get_visible_counts(&map);

    let max = counts.iter()
        .max_by_key(|(_, c)| c)
        .unwrap();

    println!("{:#?}", max);
}

fn collect_same_angle_asteroids(map: &Vec<Point>, base: &Point) -> Vec<(f64, Vec<Point>)> {

    let mut collections: Vec<(f64, Vec<Point>)> = vec![];

    'outer: for asteroid in map {

        if asteroid == base { continue; }

        for (_, col) in &mut collections {
            if col[0].same_angle(base, asteroid) {
                col.push(*asteroid);
                col.sort_by(|a, b|
                    a.dist(base)
                        .partial_cmp(&b.dist(base))
                        .unwrap()
                );
                continue 'outer;
            }
        }

        collections.push((asteroid.angle_from(base), vec![*asteroid]));
    }

    // Sort by angle, then reverse since the laser rotates clockwise
    collections.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    collections
}

pub fn part2() {

    let map = parse_map(MAP);

    let counts = get_visible_counts(&map);

    let base = counts.iter()
        .max_by_key(|(_, c)| c)
        .unwrap();

    let mut cols = collect_same_angle_asteroids(&map, &base.0);

    // for col in cols {
    //     println!("{:?}", col);
    // }

    let mut counter = 0;
    'outer: loop {
        for (_, col) in &mut cols {
            if col.len() > 0 {
                // Vaporize!
                println!("{} => {:?}", counter, col[0]);
                col.remove(0);
                counter += 1;
                if counter == 200 {
                    break 'outer;
                }
            }
        }
    }
}
