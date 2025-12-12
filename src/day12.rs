#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(usize, usize, Vec<usize>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .skip(30)
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap();
            let (w, h) = size.split_once('x').unwrap();
            let counts = counts.split(' ').map(|c| c.parse::<usize>().unwrap()).collect();
            (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap(), counts)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|&&(w, h, ref counts)| {
            let available = (w / 3) * (h / 3);
            let needed = counts.iter().sum::<usize>();
            available >= needed
        })
        .count()
}

pub fn part2(input: &Input) -> usize {
    0
}
