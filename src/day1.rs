#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(bool, usize)>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| (line.starts_with('R'), line[1..].parse().unwrap())).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut cnt = 0;
    let mut curr = 50;

    for &(right, n) in input {
        let n = if right { n } else { 100 - (n % 100) };
        curr = (curr + n) % 100;
        cnt += (curr == 0) as usize
    }

    cnt
}

pub fn part2(input: &Input) -> usize {
    let mut cnt = 0;
    let mut curr = 50;

    for &(right, mut n) in input {
        while n >= 100 {
            n -= 100;
            cnt += 1;
        }
        if n != 0 {
            if right {
                cnt += (curr + n >= 100) as usize;
            } else if curr > 0 {
                cnt += (n >= curr) as usize;
            }
        }
        let n = if right { n } else { 100 - (n % 100) };
        curr = (curr + n) % 100;
    }

    cnt
}
