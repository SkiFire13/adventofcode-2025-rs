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

pub fn part2(input: &Input) -> usize {
    fn solve_rec(
        buttons: &[Vec<usize>],
        goal: &[usize],
        mut steps: usize,
        mut best: Result<usize, ()>,
        used: &mut [bool],
    ) -> Result<usize, ()> {
        let buttons_can_add = buttons
            .iter()
            .enumerate()
            .map(|(j, button)| (j, &**button))
            .filter(|&(_, button)| button.iter().all(|&b| goal[b] != 0));

        if Ok(steps) >= best {
            return best;
        }

        let mut target = usize::MAX;
        let mut target_count = usize::MAX;
        let mut target_exclude = usize::MAX;

        for i in 0..goal.len() {
            if goal[i] == 0 {
                continue;
            }

            for j in 0..goal.len() {
                if goal[j] == 0 || goal[j] >= goal[i] {
                    continue;
                }

                let count = buttons_can_add
                    .clone()
                    .filter(|&(k, button)| !used[k] && button.contains(&i) && !button.contains(&j))
                    .count();

                if count == 0 {
                    return best;
                }

                if count < target_count {
                    target = i;
                    target_count = count;
                    target_exclude = j;
                }
            }

            if target != i {
                let count = buttons_can_add
                    .clone()
                    .filter(|&(j, button)| !used[j] && button.contains(&i))
                    .count();

                if count == 0 {
                    return best;
                }

                if count < target_count || (count == 1 && target_exclude != usize::MAX) {
                    target = i;
                    target_count = count;
                    target_exclude = usize::MAX;
                }
            }
        }

        if target == usize::MAX {
            return best.min(Ok(steps));
        }

        let max_remaining_steps = best.unwrap_or(usize::MAX) - steps;

        for (i, button) in buttons_can_add {
            if used[i] || !(button.contains(&target) && !button.contains(&target_exclude)) {
                continue;
            }

            let min = (target_count == 1)
                .then(|| goal[target] - goal.get(target_exclude).unwrap_or(&0))
                .unwrap_or(0);
            let max = button.iter().map(|&b| goal[b]).min().unwrap().min(max_remaining_steps);

            if min > max {
                continue;
            }

            let mut new_goal = goal.to_vec();
            button.iter().for_each(|&b| new_goal[b] -= min);
            steps += min;

            used[i] = true;
            for j in min..max + 1 {
                best = solve_rec(buttons, &new_goal, steps, best, used);

                if j != max {
                    button.iter().for_each(|&b| new_goal[b] -= 1);
                    steps += 1;
                }
            }
            used[i] = false;

            break;
        }

        best
    }

    let mut i = 0;
    let mut now = std::time::Instant::now();

    input
        .iter()
        .map(|(_, buttons, joltages)| {
            solve_rec(buttons, joltages, 0, Err(()), &mut vec![false; buttons.len()]).unwrap()
        })
        .inspect(|res| {
            println!("{i:<3}: {res} in {:?}", now.elapsed());
            i += 1;
            now = std::time::Instant::now();
        })
        .sum()
}
