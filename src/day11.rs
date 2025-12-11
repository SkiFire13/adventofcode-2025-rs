#[allow(unused_imports)]
use super::prelude::*;
type Input<'i> = HashMap<&'i str, Vec<&'i str>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (src, dests) = line.split_once(": ").unwrap();
            (src, dests.split(' ').collect())
        })
        .collect()
}

fn solve_rec<'i>(src: &'i str, input: &Input<'i>, cache: &mut HashMap<&'i str, usize>) -> usize {
    if let Some(&res) = cache.get(src) {
        return res;
    }

    let Some(edges) = input.get(&src) else { return 0 };
    let res = edges.iter().map(|e| solve_rec(e, input, cache)).sum();
    cache.insert(src, res);

    res
}

pub fn part1(input: &Input) -> usize {
    solve_rec("you", input, &mut HashMap::from([("out", 1)]))
}

pub fn part2(input: &Input) -> usize {
    let svr_dac_fft_out = solve_rec("svr", input, &mut HashMap::from([("dac", 1)]))
        * solve_rec("dac", input, &mut HashMap::from([("fft", 1)]))
        * solve_rec("fft", input, &mut HashMap::from([("out", 1)]));

    let svr_fft_dac_out = solve_rec("svr", input, &mut HashMap::from([("fft", 1)]))
        * solve_rec("fft", input, &mut HashMap::from([("dac", 1)]))
        * solve_rec("dac", input, &mut HashMap::from([("out", 1)]));

    svr_dac_fft_out + svr_fft_dac_out
}
