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

    let mut tot = 0;

    for &i in available {
        let mut spoiled = true;
        for &(s, e) in fresh {
            if s <= i && i <= e {
                spoiled = false;
            }
        }
        if !spoiled {
            tot += 1;
        }
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let (fresh, _) = input;

    let mut end = 0;
    let mut tot = 0;

    for &(mut s, mut e) in fresh {
        s = s.max(end);
        e = (e + 1).max(end);
        tot += e - s;
        end = e;
    }

    tot
}
