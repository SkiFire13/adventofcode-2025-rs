#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<(usize, usize)>, Vec<usize>);

pub fn input_generator(input: &str) -> Input {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let fresh = fresh
        .lines()
        .map(|f| {
            let (s, e) = f.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .sorted()
        .collect();
    let available = available.lines().map(|a| a.parse().unwrap()).collect();
    (fresh, available)
}

pub fn part1(input: &Input) -> usize {
    let (fresh, available) = input;
    available.iter().filter(|&&i| fresh.iter().any(|&(s, e)| s <= i && i <= e)).count()
}

pub fn part2(input: &Input) -> usize {
    let (fresh, _) = input;

    let mut end = 0;
    let mut tot = 0;

    for &(mut s, e) in fresh {
        s = end.max(s);
        end = end.max(e + 1);
        tot += end - s;
    }

    tot
}
