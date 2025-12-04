#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |b, _, _| b as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut count = 0;

    for (x, y) in input.iter_by_row() {
        if input[(x, y)] != b'@' {
            continue;
        }
        let mut nc = 0;
        for (nx, ny) in input.square_neighbours((x, y)) {
            if input[(nx, ny)] == b'@' {
                nc += 1;
            }
        }

        if nc < 4 {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut removed = 0;

    loop {
        let mut removable = Vec::new();

        for (x, y) in input.iter_by_row() {
            if input[(x, y)] != b'@' {
                continue;
            }

            let mut nc = 0;
            for (nx, ny) in input.square_neighbours((x, y)) {
                if input[(nx, ny)] == b'@' {
                    nc += 1;
                }
            }

            if nc < 4 {
                removable.push((x, y));
            }
        }

        if removable.is_empty() {
            return removed;
        }

        removed += removable.len();
        for (x, y) in removable {
            input[(x, y)] = b'.';
        }
    }
}
