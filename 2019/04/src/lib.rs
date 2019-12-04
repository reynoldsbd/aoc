use std::collections::HashMap;

fn is_valid_1(num: usize) -> bool {

    let mut last_digit = None;
    let mut decreases = false;
    let mut has_adj = false;

    for digit in num.to_string().chars() {
        let digit: usize = digit.to_string()
            .parse()
            .unwrap();

        if let Some(last_digit) = last_digit {
            if digit < last_digit {
                decreases = true;
                break;
            } else if digit == last_digit {
                has_adj = true;
            }
        }

        last_digit = Some(digit);
    }

    !decreases && has_adj
}

fn is_valid_2(num: usize) -> bool {

    let mut last_digit = None;
    let mut decreases = false;
    let mut adj_counts: HashMap<usize, usize> = HashMap::new();

    for digit in num.to_string().chars() {
        let digit: usize = digit.to_string()
            .parse()
            .unwrap();

        if let Some(last_digit) = last_digit {
            if digit < last_digit {
                decreases = true;
                break;
            } else if digit == last_digit {
                let entry = adj_counts.entry(digit)
                    .or_insert(1);
                *entry += 1;
            }
        }

        last_digit = Some(digit);
    }

    !decreases && adj_counts.iter().any(|(_, c)| *c == 2)
}

pub fn part1() {

    let num_valid: usize = (109165..576723)
        .filter(|n| is_valid_1(*n))
        .count();

    println!("{}", num_valid);
}

pub fn part2() {

    let num_valid: usize = (109165..576723)
        .filter(|n| is_valid_2(*n))
        .count();

    println!("{}", num_valid);
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn part1_case0() {
        assert!(is_valid_1(111111))
    }

    #[test]
    fn part1_case1() {
        assert!(!is_valid_1(223450))
    }

    #[test]
    fn part1_case2() {
        assert!(!is_valid_1(123789))
    }

    #[test]
    fn part2_case0() {
        assert!(is_valid_2(112233))
    }

    #[test]
    fn part2_case1() {
        assert!(!is_valid_2(123444))
    }

    #[test]
    fn part2_case2() {
        assert!(is_valid_2(111122))
    }
}
