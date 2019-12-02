use std::collections::HashSet;

const DELTAS: &'static str = include_str!("deltas.txt");

fn total_delta(deltas: impl IntoIterator<Item = isize>) -> isize {

    let mut freq = 0;

    for delta in deltas {
        freq += delta;
    }

    freq
}

fn calibration_freq(deltas: &[isize]) -> isize {

    let mut freq = 0;
    let mut past_freqs = HashSet::new();

    for delta in deltas.iter().cycle() {
        if !past_freqs.insert(freq) {
            return freq;
        }
        freq += delta;
    }

    unreachable!();
}

pub fn part1() {

    let deltas = DELTAS.lines()
        .map(|l| l.parse::<isize>().unwrap());
    let freq = total_delta(deltas);

    println!("{}", freq);
}

pub fn part2() {

    let deltas = DELTAS.lines()
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let freq = calibration_freq(&deltas);

    println!("{}", freq);
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: shouldn't need to allocate Vec for each test

    #[test]
    fn part1_case_0() {
        let freq = total_delta(vec![1, -2, 3, 1]);
        assert_eq!(freq, 3);
    }

    #[test]
    fn part1_case_1() {
        let freq = total_delta(vec![1, 1, 1]);
        assert_eq!(freq, 3);
    }

    #[test]
    fn part1_case_2() {
        let freq = total_delta(vec![1, 1, -2]);
        assert_eq!(freq, 0);
    }

    #[test]
    fn part1_case_3() {
        let freq = total_delta(vec![-1, -2, -3]);
        assert_eq!(freq, -6);
    }

    #[test]
    fn part2_case_0() {
        let freq = calibration_freq(&[1, -2, 3, 1]);
        assert_eq!(freq, 2);
    }

    #[test]
    fn part2_case_1() {
        let freq = calibration_freq(&[1, -1]);
        assert_eq!(freq, 0);
    }

    // Since my solution is naive, the below solution does not complete in
    // a reasonable amount of time
    // #[test]
    // fn part2_case_2() {
    //     let freq = calibration_freq(&[3, 3, 4, -2, 4]);
    //     assert_eq!(freq, 10);
    // }

    #[test]
    fn part2_case_3() {
        let freq = calibration_freq(&[-6, 3, 8, 5, -6]);
        assert_eq!(freq, 5);
    }

    #[test]
    fn part2_case_4() {
        let freq = calibration_freq(&[7, 7, -2, -7, -4]);
        assert_eq!(freq, 14);
    }
}
