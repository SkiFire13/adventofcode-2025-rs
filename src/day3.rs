#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u8>>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.chars().map(|c| c as u8 - b'0').collect()).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut tot = 0;

    for line in input {
        let mut im = 0;
        for i in 0..line.len() - 1 {
            if line[i] > line[im] {
                im = i;
            }
        }

        let i = line[im];
        let j = line[im + 1..].iter().copied().max().unwrap();
        tot += (10 * i + j) as usize;
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let mut tot = 0;

    fn max_pos(s: &[u8]) -> usize {
        let mut im = 0;
        for i in 0..s.len() {
            if s[i] > s[im] {
                im = i;
            }
        }
        im
    }

    for line in input {
        let p1 = max_pos(&line[..line.len() - 11]);
        let p2 = p1 + 1 + max_pos(&line[p1 + 1..line.len() - 10]);
        let p3 = p2 + 1 + max_pos(&line[p2 + 1..line.len() - 9]);
        let p4 = p3 + 1 + max_pos(&line[p3 + 1..line.len() - 8]);
        let p5 = p4 + 1 + max_pos(&line[p4 + 1..line.len() - 7]);
        let p6 = p5 + 1 + max_pos(&line[p5 + 1..line.len() - 6]);
        let p7 = p6 + 1 + max_pos(&line[p6 + 1..line.len() - 5]);
        let p8 = p7 + 1 + max_pos(&line[p7 + 1..line.len() - 4]);
        let p9 = p8 + 1 + max_pos(&line[p8 + 1..line.len() - 3]);
        let p10 = p9 + 1 + max_pos(&line[p9 + 1..line.len() - 2]);
        let p11 = p10 + 1 + max_pos(&line[p10 + 1..line.len() - 1]);
        let p12 = p11 + 1 + max_pos(&line[p11 + 1..]);

        let mut n = 0;
        n = 10 * n + line[p1] as usize;
        n = 10 * n + line[p2] as usize;
        n = 10 * n + line[p3] as usize;
        n = 10 * n + line[p4] as usize;
        n = 10 * n + line[p5] as usize;
        n = 10 * n + line[p6] as usize;
        n = 10 * n + line[p7] as usize;
        n = 10 * n + line[p8] as usize;
        n = 10 * n + line[p9] as usize;
        n = 10 * n + line[p10] as usize;
        n = 10 * n + line[p11] as usize;
        n = 10 * n + line[p12] as usize;

        tot += n;
    }

    tot
}
