#[allow(unused_imports)]
use super::prelude::*;
type Input<'i> = [&'i str; 5];

pub fn input_generator(input: &str) -> Input {
    input.lines().collect::<Vec<_>>().try_into().unwrap()
}

pub fn part1(input: &Input) -> usize {
    let &[l1, l2, l3, l4, lo] = input;

    let n1 = l1.split_whitespace().map(|n| n.parse::<usize>().unwrap());
    let n2 = l2.split_whitespace().map(|n| n.parse::<usize>().unwrap());
    let n3 = l3.split_whitespace().map(|n| n.parse::<usize>().unwrap());
    let n4 = l4.split_whitespace().map(|n| n.parse::<usize>().unwrap());
    let op = lo.split_whitespace();

    let mut tot = 0;
    for (n1, n2, n3, n4, op) in izip!(n1, n2, n3, n4, op) {
        match op {
            "+" => tot += n1 + n2 + n3 + n4,
            "*" => tot += n1 * n2 * n3 * n4,
            _ => unreachable!(),
        }
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let lines = input.map(str::as_bytes);
    let len = lines[0].len();

    let mut tot = 0;
    let mut i = 0;

    fn parse(a: [u8; 5]) -> Option<usize> {
        a[..4]
            .iter()
            .filter(|&&d| d != b' ')
            .map(|&d| d.saturating_sub(b'0') as usize)
            .reduce(|acc, d| 10 * acc + d)
    }

    while i < len {
        let a = lines.map(|l| l[i]);
        let mut acc = parse(a).unwrap();
        let op = a[4];
        i += 1;

        while i < len {
            let a = lines.map(|l| l[i]);
            let Some(n) = parse(a) else { break };
            match op {
                b'+' => acc += n,
                b'*' => acc *= n,
                _ => unreachable!(),
            }
            i += 1;
        }

        tot += acc;
        i += 1;
    }

    tot
}
