#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().map(|c| c as u8 - b'0').collect()).collect()
}

fn solve(line: &[u8], n: usize) -> usize {
    let mut stack = Vec::with_capacity(n);

    for (i, &b) in line.iter().enumerate() {
        let remaining = line.len() - i - 1;
        while let Some(&last) = stack.last()
            && last < b
            && stack.len() + remaining >= n
        {
            stack.pop();
        }
        if stack.len() < n {
            stack.push(b);
        }
    }

    stack.into_iter().fold(0, |acc, d| 10 * acc + d as usize)
}

pub fn part1(input: &Input) -> usize {
    input.iter().map(|line| solve(line, 2)).sum()
}

pub fn part2(input: &Input) -> usize {
    input.iter().map(|line| solve(line, 12)).sum()
}
