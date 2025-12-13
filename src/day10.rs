#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (lights, rest) = line[1..].split_once("] (").unwrap();
            let lights = lights.chars().map(|c| c == '#').collect();

            let (buttons, rest) = rest.split_once(") {").unwrap();
            let buttons = buttons
                .split(") (")
                .map(|bs| bs.split(',').map(|b| b.parse().unwrap()).collect())
                .collect();

            let joltages = rest[..rest.len() - 1].split(',').map(|j| j.parse().unwrap()).collect();

            (lights, buttons, joltages)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(lights, buttons, _)| {
            let mut seen = HashSet::from([lights.to_vec()]);
            let mut frontier = Vec::from([lights.to_vec()]);
            let mut steps = 0;

            'search: loop {
                steps += 1;

                let mut next = Vec::new();
                for lights in frontier {
                    for button in buttons {
                        let mut lights = lights.clone();
                        for &i in button {
                            lights[i] ^= true;
                        }
                        if lights.iter().all(|&l| l == false) {
                            break 'search;
                        }
                        if !seen.contains(&lights) {
                            seen.insert(lights.to_vec());
                            next.push(lights);
                        }
                    }
                }
                frontier = next;
            }

            steps
        })
        .sum()
}

pub fn part2(input: &Input) -> i32 {
    input
        .iter()
        .map(|(_, buttons, joltages)| {
            let max_value = joltages.iter().copied().max().unwrap() as i32;

            // Create the matrix for the system of linear equations
            let cols = buttons.len() + 1;
            let rows = joltages.len();
            let mut matrix = vec![0; cols * rows];
            for (i, button) in buttons.iter().enumerate() {
                for &b in button {
                    matrix[b * cols + i] = 1;
                }
            }
            for (i, &j) in joltages.iter().enumerate() {
                matrix[i * cols + (cols - 1)] = j as i32;
            }

            // Put the matrix into integer RREF
            let mut pivot = 0;
            for c in 0..cols - 1 {
                let Some(row) = (pivot..rows).find(|&row| matrix[row * cols + c] != 0) else {
                    continue;
                };

                if pivot != row {
                    (0..cols).for_each(|c| matrix.swap(pivot * cols + c, row * cols + c));
                }
                if pivot != c {
                    (0..rows).for_each(|r| matrix.swap(r * cols + c, r * cols + pivot));
                }
                let pivot_val = matrix[pivot * cols + pivot];

                for r in 0..rows {
                    if r == pivot {
                        continue;
                    }
                    let factor = matrix[r * cols + pivot];
                    if factor != 0 {
                        for k in 0..cols {
                            matrix[r * cols + k] *= pivot_val;
                            matrix[r * cols + k] -= matrix[pivot * cols + k] * factor;
                        }
                    }
                }

                pivot += 1;
                if pivot >= rows {
                    break;
                }
            }

            // Find the free variables.
            let mut vars = Vec::new();
            'c: for _ in pivot..cols - 1 {
                // Prefer variables in equations with no other unchosen free variables.
                // These equations allows us to get some nice bounds on the free variable later on.
                'r: for r in 0..rows {
                    let mut k = 0;
                    for c in pivot..cols - 1 {
                        if matrix[r * cols + c] != 0 && !vars.contains(&c) {
                            if k != 0 {
                                continue 'r;
                            }
                            k = c;
                        }
                    }
                    if k != 0 {
                        vars.push(k);
                        continue 'c;
                    }
                }

                // If the above fails, pick a variable present in the most equations
                // to increase the chance that this allows another free variable to be chosen
                // with the first way.
                let mut best_c = pivot;
                let mut best_count = 0;
                for c in pivot..cols - 1 {
                    if !vars.contains(&c) {
                        let mut count = 0;
                        for r in 0..rows {
                            if matrix[r * cols + c] != 0 {
                                count += 1;
                            }
                        }
                        if count > best_count {
                            best_c = c;
                            best_count = count;
                        }
                    }
                }
                vars.push(best_c);
            }

            // Solve recursively for all free variables.
            fn solve_rec(
                vars: &[usize],
                values: &mut [i32],
                matrix: &[i32],
                cols: usize,
                rows: usize,
                mut best: Result<i32, ()>,
                max_value: i32,
            ) -> Result<i32, ()> {
                // If we have no more free variables compute the total.
                if vars.len() == 0 {
                    let mut tot = values.iter().sum();
                    for r in 0..cols - 1 - values.len() {
                        let mut sum = matrix[r * cols + (cols - 1)];
                        for i in 0..values.len() {
                            let c = cols - 1 - values.len() + i;
                            sum -= matrix[r * cols + c] * values[i];
                        }
                        if sum % matrix[r * cols + r] != 0 {
                            return best;
                        }
                        sum /= matrix[r * cols + r];
                        if sum < 0 {
                            return best;
                        }
                        tot += sum;
                    }
                    return best.min(Ok(tot));
                }

                let x = vars[0];

                let mut min = 0;
                let mut max = max_value;

                // Go through each equation where this appears as the only free variable
                // and use that to get a bound for it.
                'r: for r in 0..rows {
                    if matrix[r * cols + x] == 0 {
                        continue;
                    }

                    let mut n = matrix[r * cols + r];
                    let m = matrix[r * cols + x];
                    let mut rhs = matrix[r * cols + (cols - 1)];
                    for i in 0..values.len() {
                        let c = cols - 1 - values.len() + i;
                        if c != x && matrix[r * cols + c] != 0 {
                            if vars.contains(&c) {
                                if (matrix[r * cols + c] > 0) == (n > 0) {
                                    n += matrix[r * cols + c];
                                } else {
                                    continue 'r;
                                }
                            } else {
                                rhs -= matrix[r * cols + c] * values[i];
                            }
                        }
                    }

                    // Require the other variables on this line to be >= 0 and <= max_value
                    if (n > 0) ^ (m > 0) {
                        min = min.max(rhs / m);
                        max = max.min((rhs - max_value * n) / m);
                    } else {
                        max = max.min(rhs / m);
                        min = min.max((rhs - max_value * n + (m - 1)) / m);
                    }
                }

                // Go through all possible values for this variable.
                let mut v = min;
                while v <= max {
                    values[x - (cols - 1 - values.len())] = v;
                    best = solve_rec(&vars[1..], values, matrix, cols, rows, best, max_value);
                    if let Ok(best) = best {
                        max = max.min(best);
                    }
                    v += 1;
                }

                best
            }

            solve_rec(&vars, &mut vec![0; vars.len()], &matrix, cols, rows, Err(()), max_value)
                .unwrap()
        })
        .sum()
}
