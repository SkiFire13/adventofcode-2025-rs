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

        let min_add_steps = goal.iter().copied().max().unwrap();

        if min_add_steps == 0 {
            return best.min(Ok(steps));
        }

        if Ok(steps + min_add_steps) >= best {
            return best;
        }

        let mut target = usize::MAX;
        let mut target_count = usize::MAX;

        for i in 0..goal.len() {
            if goal[i] == 0 {
                continue;
            }

            for j in 0..goal.len() {
                if goal[j] == 0 {
                    continue;
                }

                if goal[j] < goal[i] {
                    if let Ok((_, button)) = buttons_can_add
                        .clone()
                        .filter(|&(k, _)| !used[k])
                        .filter(|&(_, button)| button.contains(&i) && !button.contains(&j))
                        .exactly_one()
                    {
                        let mut new_goal = goal.to_vec();
                        let n = goal[i] - goal[j];
                        for &b in button {
                            if goal[b] < n {
                                return best;
                            }

                            new_goal[b] -= n;
                        }

                        return solve_rec(buttons, &new_goal, steps + n, best, used);
                    }
                }
            }

            if let Ok((_, button)) = buttons_can_add
                .clone()
                .filter(|&(k, _)| !used[k])
                .filter(|(_, button)| button.contains(&i))
                .exactly_one()
            {
                let mut new_goal = goal.to_vec();
                let n = goal[i];
                for &b in button {
                    if goal[b] < n {
                        return best;
                    }

                    new_goal[b] -= n;
                }

                return solve_rec(buttons, &new_goal, steps + n, best, used);
            }

            let button_options = buttons_can_add
                .clone()
                .filter(|&(k, _)| !used[k])
                .filter(|(_, button)| button.contains(&i))
                .count();

            if button_options == 0 {
                return best;
            }

            let count = button_options * goal[i];

            if count < target_count {
                target = i;
                target_count = count;
            }
        }

        if target == usize::MAX {
            return best.min(Ok(steps));
        }

        for (i, button) in buttons_can_add {
            if used[i] || !(button.contains(&target)) {
                continue;
            }

            let min = 0;
            let max = button.iter().map(|&b| goal[b]).min().unwrap();

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

    input
        .par_iter()
        .map(|(_, buttons, joltages)| {
            solve_rec(buttons, joltages, 0, Err(()), &mut vec![false; buttons.len()]).unwrap()
        })
        .sum()
}
