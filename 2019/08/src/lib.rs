use std::convert::TryInto;

struct Image(Vec<Vec<Vec<u8>>>);

impl Image {
    fn parse(s: &str, width: usize, height: usize) -> Self {


        let mut cur_width = 0;
        let mut cur_height = 0;

        let mut layers = vec![];
        let mut cur_row = vec![];
        let mut cur_layer = vec![];

        for digit in s.trim().chars() {

            cur_row.push(digit.to_string().parse().unwrap());
            cur_width += 1;

            if cur_width >= width {
                cur_layer.push(cur_row);
                cur_row = vec![];
                cur_height += 1;
                cur_width = 0;
            }

            if cur_height >= height {
                layers.push(cur_layer);
                cur_layer = vec![];
                cur_height = 0;
            }
        }

        Image(layers)
    }

    fn render(&self) {

        let width = self.0[0][0].len();
        let height = self.0[0].len();

        let mut img = vec![];
        for row_idx in 0..height {
            img.push(vec![]);
            for _ in 0..width {
                img[row_idx].push('_');
            }
        }

        for layer in &self.0 {
            for row_idx in 0..height {
                for pix_idx in 0..width {
                    let pix = match layer[row_idx][pix_idx] {
                        0 => ' ',
                        1 => 'X',
                        2 => '_',
                        _ => panic!("unrecognized pixel"),
                    };

                    if img[row_idx][pix_idx] == '_' {
                        img[row_idx][pix_idx] = pix;
                    }
                }
            }
        }

        use std::iter::FromIterator;
        for row in img {
            let row = String::from_iter(row);
            println!("{}", row);
        }
    }
}

fn count_digit(layer: &Vec<Vec<u8>>, digit: u8) -> usize {

    let mut num = 0;

    for row in layer {
        for pix in row {
            if *pix == digit {
                num += 1;
            }
        }
    }

    num
}

fn fewest_zero_layer(img: &Image) -> usize {

    let mut min_zeros = None;
    let mut min_layer = None;

    for (idx, layer) in img.0.iter().enumerate() {
        let mut num_zeros = 0;
        for row in layer {
            for pix in row {
                if *pix == 0 {
                    num_zeros += 1;
                }
            }
        }

        if min_zeros.is_none() || min_zeros.unwrap() > num_zeros {
            min_zeros = Some(num_zeros);
            min_layer = Some(idx);
        }
    }

    min_layer.unwrap()
}

const IMG: &'static str = include_str!("img.txt");

pub fn part1() {

    println!("pixels: {}", IMG.len() - 1);

    let img = Image::parse(IMG, 25, 6);

    println!("layers: {}", img.0.len());

    let layer = fewest_zero_layer(&img);

    println!("layer: {}", layer);

    let num_ones = count_digit(&img.0[layer], 1);
    let num_twos = count_digit(&img.0[layer], 2);

    println!("{}", dbg!(num_ones) * dbg!(num_twos));
}

pub fn part2() {

    let img = Image::parse(IMG, 25, 6);

    img.render();
}
