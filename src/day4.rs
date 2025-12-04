#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |b, _, _| b as u8)
}

pub fn part1(input: &Input) -> usize {
    input
        .iter_by_row()
        .filter(|&p| input[p] == b'@')
        .filter(|&p| input.square_neighbours(p).filter(|&q| input[q] == b'@').count() < 4)
        .count()
}

pub fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut removed = 0;

    loop {
        let removable = input
            .iter_by_row()
            .filter(|&p| input[p] == b'@')
            .filter(|&p| input.square_neighbours(p).filter(|&q| input[q] == b'@').count() < 4)
            .collect::<Vec<_>>();

        if removable.is_empty() {
            return removed;
        }

        removed += removable.len();

        for p in removable {
            input[p] = b'.';
        }
    }
}
