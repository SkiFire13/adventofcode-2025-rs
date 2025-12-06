#[allow(unused_imports)]
use super::prelude::*;
type Input<'i> = (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>, Vec<u8>, &'i str);

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let n1 = lines.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
    let n2 = lines.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
    let n3 = lines.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
    let n4 = lines.next().unwrap().split_ascii_whitespace().map(|n| n.parse().unwrap()).collect();
    let op = lines.next().unwrap().split_ascii_whitespace().map(|s| s.as_bytes()[0]).collect();

    (n1, n2, n3, n4, op, input)
}

pub fn part1(input: &Input) -> usize {
    let (n1, n2, n3, n4, op, _) = input;

    let mut tot = 0;
    for (&n1, &n2, &n3, &n4, &op) in izip!(n1, n2, n3, n4, op) {
        match op {
            b'+' => tot += n1 + n2 + n3 + n4,
            b'*' => tot += n1 * n2 * n3 * n4,
            _ => unreachable!(),
        }
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let &(_, _, _, _, _, input) = input;

    let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut tot = 0;
    let mut i = 0;

    while i < lines[0].len() {
        let d1 = lines[0][i];
        let d2 = lines[1][i];
        let d3 = lines[2][i];
        let d4 = lines[3][i];

        let mut n = d1.saturating_sub(b'0') as usize;
        if d2 != b' ' {
            n = 10 * n + d2.saturating_sub(b'0') as usize
        }
        if d3 != b' ' {
            n = 10 * n + d3.saturating_sub(b'0') as usize
        }
        if d4 != b' ' {
            n = 10 * n + d4.saturating_sub(b'0') as usize
        }

        let op = lines[4][i];

        let mut acc = n;

        i += 1;
        while i < lines[0].len()
            && (lines[0][i] != b' '
                || lines[1][i] != b' '
                || lines[2][i] != b' '
                || lines[3][i] != b' ')
        {
            let d1 = lines[0][i];
            let d2 = lines[1][i];
            let d3 = lines[2][i];
            let d4 = lines[3][i];

            let mut n = d1.saturating_sub(b'0') as usize;
            if d2 != b' ' {
                n = 10 * n + d2.saturating_sub(b'0') as usize
            }
            if d3 != b' ' {
                n = 10 * n + d3.saturating_sub(b'0') as usize
            }
            if d4 != b' ' {
                n = 10 * n + d4.saturating_sub(b'0') as usize
            }

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
