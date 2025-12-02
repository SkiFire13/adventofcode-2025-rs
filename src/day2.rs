#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .trim()
        .split(',')
        .map(|r| {
            let (s, e) = r.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect()
}

const POWERS_OF_TEN: &[usize] = &[10, 100, 1000, 10000, 100000];

pub fn part1(input: &Input) -> usize {
    let mut tot = 0;
    for &(s, e) in input {
        for i in s..=e {
            if POWERS_OF_TEN.iter().any(|&d| i / d == i % d && i % d >= d / 10) {
                tot += i;
            }
        }
    }
    tot
}

pub fn part2(input: &Input) -> usize {
    fn invalid_repeated(i: usize, d: usize, b: usize) -> bool {
        i % d == b && (i / d == 0 || invalid_repeated(i / d, d, b))
    }

    let mut tot = 0;
    for &(s, e) in input {
        for i in s..=e {
            if POWERS_OF_TEN.iter().any(|&d| i % d >= d / 10 && invalid_repeated(i / d, d, i % d)) {
                tot += i;
            }
        }
    }
    tot
}
