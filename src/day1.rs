#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(bool, usize)>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| (line.starts_with('R'), line[1..].parse().unwrap())).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut cnt = 0;
    let mut curr = 50;

    for &(positive, n) in input {
        let n = if positive { n } else { 100 - (n % 100) };
        curr = (curr + n) % 100;
        cnt += (curr == 0) as usize
    }

    cnt
}

pub fn part2(input: &Input) -> usize {
    let mut cnt = 0;
    let mut curr = 50;

    for &(positive, n) in input {
        let (hundreds, rest) = (n / 100, n % 100);
        cnt += hundreds;

        if rest != 0 {
            if positive {
                cnt += (curr + rest >= 100) as usize;
            } else if curr > 0 {
                cnt += (rest >= curr) as usize;
            }

            let k = if positive { n } else { 100 - rest };
            curr = (curr + k) % 100;
        }
    }

    cnt
}
