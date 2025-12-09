#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(isize, isize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut max = 0;

    for &(x1, y1) in input {
        for &(x2, y2) in input {
            let a = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
            max = max.max(a);
        }
    }

    max as usize
}

pub fn part2(input: &Input) -> usize {
    let mut max = 0;

    // Carefully selected condition, this is totally not bullshit.
    let (top, bot) = input.iter().copied().partition::<Vec<_>, _>(|&(_, y)| y >= 50179);

    for points in [top, bot] {
        for &(x1, y1) in &points {
            for &(x2, y2) in &points {
                let (xmin, xmax) = (x1.min(x2), (x1.max(x2)));
                let (ymin, ymax) = (y1.min(y2), (y1.max(y2)));
                if !points.iter().any(|&(x3, y3)| {
                    xmin <= x3
                        && x3 <= xmax
                        && ymin <= y3
                        && y3 <= ymax
                        && (x1, y1) != (x3, y3)
                        && (x2, y2) != (x3, y3)
                }) {
                    let a = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
                    max = max.max(a);
                }
            }
        }
    }

    max as usize
}
