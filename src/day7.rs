#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();

    let mut pos = HashMap::from([(sx as isize, 1u64)]);
    let mut y = sy as isize + 1;

    let mut splits = 0;

    while y < input.h() as isize {
        let mut tmp = HashMap::new();

        for (x, c) in pos {
            if let Some(&b'^') = input.iget((x, y)) {
                *tmp.entry(x - 1).or_insert(0) += c;
                *tmp.entry(x + 1).or_insert(0) += c;
                splits += 1;
            } else {
                *tmp.entry(x).or_insert(0) += c;
            }
        }

        pos = tmp;

        y += 1;
    }

    splits
}

pub fn part2(input: &Input) -> u64 {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();

    let mut pos = HashMap::from([(sx as isize, 1u64)]);
    let mut y = sy as isize + 1;

    let mut splits = 1;

    while y < input.h() as isize {
        let mut tmp = HashMap::new();

        for (x, c) in pos {
            if let Some(&b'^') = input.iget((x, y)) {
                *tmp.entry(x - 1).or_insert(0) += c;
                *tmp.entry(x + 1).or_insert(0) += c;
                splits += c;
            } else {
                *tmp.entry(x).or_insert(0) += c;
            }
        }

        pos = tmp;

        y += 1;
    }

    splits
}
