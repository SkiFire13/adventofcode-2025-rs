#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i64, i64, i64)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect_tuple().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> i32 {
    let distance = |i: usize, j: usize| {
        let (x1, y1, z1) = input[i];
        let (x2, y2, z2) = input[j];
        (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)
    };

    let edges = (0..input.len())
        .flat_map(|i| (i + 1..input.len()).map(move |j| (i, j)))
        .map(|(i, j)| (distance(i, j), i, j));

    let mut uf = vec![-1; input.len()];

    for (_, i, j) in edges.k_smallest(1000) {
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
        uf[rj] = ri as i32;
    }

    uf.iter().copied().filter(|&n| n < 0).k_smallest(3).map(|n| -n).product()
}

pub fn part2(input: &Input) -> i64 {
    let mut distances = vec![i64::MAX; input.len()];

    let mut next = 0;
    for _ in 0..input.len() - 1 {
        distances[next] = -1;
        let (x1, y1, z1) = input[next];
        let mut best = i64::MAX;
        for (i, &(x2, y2, z2)) in input.iter().enumerate() {
            if distances[i] == -1 {
                continue;
            }

            let d = (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2);
            distances[i] = distances[i].min(d);

            if distances[i] < best {
                best = distances[i];
                next = i;
            }
        }
    }

    let last = next;
    let (x1, y1, z1) = input[last];

    let mut best = i64::MAX;
    let mut other_x = 0;
    for (i, &(x2, y2, z2)) in input.iter().enumerate() {
        if i == last {
            continue;
        }

        let d = (x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2);
        if d < best {
            best = d;
            other_x = x2;
        }
    }

    x1 * other_x
}
