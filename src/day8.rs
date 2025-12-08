#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(isize, isize, isize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_tuple().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut distances = Vec::with_capacity(input.len() * input.len());

    for (i, &(x1, y1, z1)) in input.iter().enumerate() {
        for (j, &(x2, y2, z2)) in input[i + 1..].iter().enumerate() {
            let j = i + 1 + j;
            distances.push(((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2), i, j));
        }
    }

    distances.sort_unstable();

    let mut uf = vec![-1; input.len()];

    for &(_, i, j) in &distances[..1000] {
        let mut ri = i;
        while uf[ri] >= 0 {
            ri = uf[ri] as usize;
        }

        let mut rj = j;
        while uf[rj] >= 0 {
            rj = uf[rj] as usize;
        }

        if ri == rj {
            continue;
        }

        if uf[ri] > uf[rj] {
            (ri, rj) = (rj, ri);
        }

        uf[ri] += uf[rj];
        uf[rj] = ri as isize;
    }

    uf.iter().copied().k_smallest(3).filter(|&n| n < 0).map(|n| -n as usize).product::<usize>()
}

pub fn part2(input: &Input) -> usize {
    let mut distances = Vec::with_capacity(input.len() * input.len());

    for (i, &(x1, y1, z1)) in input.iter().enumerate() {
        for (j, &(x2, y2, z2)) in input[i + 1..].iter().enumerate() {
            let j = i + 1 + j;
            distances.push(((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2), i, j));
        }
    }

    distances.sort_unstable();

    let mut uf = vec![-1; input.len()];
    let mut components = input.len();

    for &(_, i, j) in &distances {
        let mut ri = i;
        while uf[ri] >= 0 {
            ri = uf[ri] as usize;
        }

        let mut rj = j;
        while uf[rj] >= 0 {
            rj = uf[rj] as usize;
        }

        if ri == rj {
            continue;
        }

        components -= 1;
        if components == 1 {
            return (input[i].0 * input[j].0) as usize;
        }

        if uf[ri] > uf[rj] {
            (ri, rj) = (rj, ri);
        }

        uf[ri] += uf[rj];
        uf[rj] = ri as isize;
    }

    unreachable!();
}
