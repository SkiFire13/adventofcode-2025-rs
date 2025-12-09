#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i64, i64)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split(',').map(|p| p.parse().unwrap()).collect_tuple().unwrap())
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut max = 0;

    for (i, &(x1, y1)) in input.iter().enumerate() {
        for &(x2, y2) in &input[i + 1..] {
            let a = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
            max = max.max(a);
        }
    }

    max
}

pub fn part2(input: &Input) -> i64 {
    let edges = input.iter().copied().tuple_windows().chain(input.last().map(|&p| (p, input[0])));

    let (mut vertical_edges, mut horizontal_edges) = edges
        .partition_map::<Vec<_>, Vec<_>, _, _, _>(|((x1, y1), (x2, y2))| {
            if x1 == x2 { Either::Left((x1, y1, y2)) } else { Either::Right((y1, x1, x2)) }
        });

    vertical_edges.sort_unstable();
    horizontal_edges.sort_unstable();

    let (_, tx1, tx2) = horizontal_edges[0];
    let clockwise = tx1 < tx2;

    let mut max = 0;

    for (i, &(x1, y1)) in input.iter().enumerate() {
        'rect: for &(x2, y2) in &input[i + 1..] {
            let (xmin, xmax, ymin, ymax) = (x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2));

            let a = (xmax - xmin + 1) * (ymax - ymin + 1);
            if a < max {
                continue 'rect;
            }

            let s = horizontal_edges.partition_point(|&(y, _, _)| y + 1 < ymin);
            let horizontal_edges = &horizontal_edges[s..];
            let e = horizontal_edges.partition_point(|&(y, _, _)| y - 1 <= ymax);
            let horizontal_edges = &horizontal_edges[..e];

            for &(mut y3, xa3, xb3) in horizontal_edges {
                y3 += if (xa3 < xb3) == clockwise { -1 } else { 1 };

                let (xl, xr) = (xa3.min(xb3), xa3.max(xb3));
                if ymin <= y3 && y3 <= ymax && xmin < xr && xl < xmax {
                    continue 'rect;
                }
            }

            let s = vertical_edges.partition_point(|&(x, _, _)| x + 1 < xmin);
            let vertical_edges = &vertical_edges[s..];
            let e = vertical_edges.partition_point(|&(x, _, _)| x - 1 <= xmax);
            let vertical_edges = &vertical_edges[..e];

            for &(mut x3, ya3, yb3) in vertical_edges {
                x3 += if (ya3 < yb3) == clockwise { 1 } else { -1 };

                let (yl, yr) = (ya3.min(yb3), ya3.max(yb3));
                if xmin <= x3 && x3 <= xmax && ymin < yr && yl < ymax {
                    continue 'rect;
                }
            }

            max = a;
        }
    }

    max
}
