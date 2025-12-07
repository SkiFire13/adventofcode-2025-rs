#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();

    let mut splits = 0;
    let mut active = vec![false; input.w()];
    active[sx] = true;

    for y in sy + 1..input.h() {
        let mut tmp = vec![false; input.w()];

        for x in 0..input.w() {
            if active[x] {
                if input[(x, y)] == b'^' {
                    tmp[x - 1] |= active[x];
                    tmp[x + 1] |= active[x];
                    splits += 1;
                } else {
                    tmp[x] |= active[x];
                }
            }
        }

        active = tmp;
    }

    splits
}

pub fn part2(input: &Input) -> u64 {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();

    let mut active = vec![0; input.w()];
    active[sx] = 1;

    for y in sy + 1..input.h() {
        let mut tmp = vec![0; input.w()];

        for x in 0..input.w() {
            if input[(x, y)] == b'^' {
                tmp[x - 1] += active[x];
                tmp[x + 1] += active[x];
            } else {
                tmp[x] += active[x];
            }
        }

        active = tmp;
    }

    active.into_iter().sum()
}
