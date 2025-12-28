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

pub fn part2(input: &Input) -> u64 {
    let mut indexes = (0..input.len()).collect::<Vec<_>>();

    // Compress x coordinates with 1-wide gap inbetween
    let mut ids_x = vec![0; input.len()];
    indexes.sort_by_key(|&i| input[i].0);
    let (mut prevx, mut previ) = (input[indexes[0]].0, 0);
    for &i in &indexes {
        previ += (input[i].0 > prevx) as usize + (input[i].0 > prevx + 1) as usize;
        prevx = input[i].0;
        ids_x[i] = previ + 1;
    }
    let w = ids_x[indexes[indexes.len() - 1]] + 1;

    // Compress y coordinates with 1-wide gap inbetween
    let mut ids_y = vec![0; input.len()];
    indexes.sort_by_key(|&i| input[i].1);
    let (mut prevy, mut previ) = (input[indexes[0]].1, 0);
    for &i in &indexes {
        previ += (input[i].1 > prevy) as usize + (input[i].1 > prevy + 1) as usize;
        prevy = input[i].1;
        ids_y[i] = previ + 1;
    }
    let h = ids_y[indexes[indexes.len() - 1]] + 1;

    // Populate the grid for edges
    let mut grid = Grid::with_dimensions_init(w, h, |_, _| 0);
    for ((&id_xa, &id_ya), (&id_xb, &id_yb)) in izip!(&ids_x, &ids_y).circular_tuple_windows() {
        let (xlow, xhigh) = (id_xa.min(id_xb), id_xa.max(id_xb));
        let (ylow, yhigh) = (id_ya.min(id_yb), id_ya.max(id_yb));
        for y in ylow..yhigh + 1 {
            for x in xlow..xhigh + 1 {
                grid[(x, y)] = 1;
            }
        }
    }

    // Flood fill the interior, starting from all top sides.
    let top_a = indexes[indexes.len() - 1];
    let top_b = indexes[indexes.len() - 2];
    let (top_a, top_b) = (min(top_a, top_b), max(top_a, top_b));
    let mut queue = Vec::new();
    for i in 1..input.len() {
        let (x1, y1) = (ids_x[i - 1], ids_y[i - 1]);
        let (x2, y2) = (ids_x[i], ids_y[i]);

        let (sx, sy) = ((x1 + x2) / 2, y1 - 1);
        if y1 == y2 && (x1 < x2) == (ids_x[top_a] < ids_x[top_b]) && grid[(sx, sy)] != 1 {
            queue.push((sx, sy));
            while let Some((x, y)) = queue.pop() {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                    if grid[(nx, ny)] != 1 {
                        grid[(nx, ny)] = 1;
                        queue.push((nx, ny));
                    }
                }
            }
        }
    }

    // Transform the grid into a 2d prefix sum.
    for y in 1..h {
        for x in 1..w {
            grid[(x, y)] += grid[(x - 1, y)] + grid[(x, y - 1)] - grid[(x - 1, y - 1)];
        }
    }

    let mut max = 0;

    // For every pair of points
    for (i, (&(xa, ya), &id_xa, &id_ya)) in izip!(input, &ids_x, &ids_y).enumerate() {
        for (&(xb, yb), &id_xb, &id_yb) in izip!(&input[i + 1..], &ids_x[i + 1..], &ids_y[i + 1..])
        {
            // If the real area does not improve over the current maximum, don't even bother.
            let area = (xa.abs_diff(xb) + 1) * (ya.abs_diff(yb) + 1);
            if area < max {
                continue;
            }

            // Compute the expected area in the compressed space.
            let (xlow, xhigh) = (id_xa.min(id_xb), id_xa.max(id_xb));
            let (ylow, yhigh) = (id_ya.min(id_yb), id_ya.max(id_yb));
            let id_area = (xhigh - xlow + 1) * (yhigh - ylow + 1);

            // Compute the actual area in the compressed space with the 2d prefix sum
            let actual_id_area = grid[(xhigh, yhigh)] + grid[(xlow - 1, ylow - 1)]
                - (grid[(xlow - 1, yhigh)] + grid[(xhigh, ylow - 1)]);

            // If they match then the rectangle between the two points is completely within the shape and thus valid.
            if id_area == actual_id_area {
                max = area;
            }
        }
    }

    max
}
