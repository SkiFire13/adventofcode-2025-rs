#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().map(|c| c as u8 - b'0').collect()).collect()
}

fn solve(mut line: &[u8], n: usize) -> usize {
    let mut sum = 0;

    for i in (0..n).rev() {
        let mut jm = 0;
        for j in 0..line.len() - i {
            if line[j] > line[jm] {
                jm = j;
            }
        }

        sum = 10 * sum + line[jm] as usize;
        line = &line[jm + 1..];
    }

    sum
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|line| solve(line, 2)).sum()
}

pub fn part2(input: &Input) -> usize {
    input.iter().map(|line| solve(line, 12)).sum()
}
