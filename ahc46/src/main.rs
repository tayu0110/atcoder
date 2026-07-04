use std::{iter::repeat, time::Instant};

use itertools::Itertools;
use proconio::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

const N: usize = 20;
const M: usize = 40;
const MAX: usize = 2 * N * M;

fn main() {
    input! {_: usize, _: usize, p: [(usize, usize); M]}

    let mut res = vec![];
    let mut ok = 0;
    for v in p.windows(2) {
        let (r, c) = v[0];
        let (i, j) = v[1];
        if r != i {
            if r > i {
                if i + 1 < r - i {
                    res.push("S U");
                    res.extend(repeat("M D").take(i));
                } else {
                    res.extend(repeat("M U").take(r - i));
                }
            } else {
                if (N - 1 - i) + 1 < i - r {
                    res.push("S D");
                    res.extend(repeat("M U").take(N - 1 - i));
                } else {
                    res.extend(repeat("M D").take(i - r));
                }
            }
        }
        if c != j {
            if c > j {
                if j + 1 < c - j {
                    res.push("S L");
                    res.extend(repeat("M R").take(j));
                } else {
                    res.extend(repeat("M L").take(c - j));
                }
            } else {
                if (N - 1 - j) + 1 < j - c {
                    res.push("S R");
                    res.extend(repeat("M L").take(N - 1 - j));
                } else {
                    res.extend(repeat("M R").take(j - c));
                }
            }
        }
        if res.len() <= MAX {
            ok += 1;
        }
    }

    let mut score = if res.len() > MAX {
        ok
    } else {
        M + MAX - res.len()
    };
    res.truncate(MAX);
    let tm = Instant::now();
    let (r, c) = p[0];
    let mut rng = thread_rng();
    dfs(
        r,
        c,
        usize::MAX,
        usize::MAX,
        &p[1..],
        &tm,
        &mut score,
        &mut rng,
        &mut [
            (0, 1, 1, "M R"),
            (0, !0, 1, "M L"),
            (1, 0, 1, "M D"),
            (!0, 0, 1, "M L"),
            (0, 1, N, "S R"),
            (0, !0, N, "S L"),
            (1, 0, N, "S D"),
            (!0, 0, N, "S U"),
        ],
        &mut vec![],
        &mut res,
    );
    println!("{}", res.into_iter().join("\n"));
}

fn dfs(
    r: usize,
    c: usize,
    pr: usize,
    pc: usize,
    p: &[(usize, usize)],
    tm: &Instant,
    score: &mut usize,
    rng: &mut ThreadRng,
    dir: &mut [(usize, usize, usize, &'static str)],
    stack: &mut Vec<&'static str>,
    res: &mut Vec<&'static str>,
) {
    if tm.elapsed().as_millis() > 1900 {
        return;
    }

    if p.is_empty() {
        if stack.len() <= MAX {
            let s = M + MAX - stack.len();
            if *score < s {
                *score = s;
                res.clear();
                res.extend(stack.iter().copied());
            }
        }
        return;
    }

    dir.shuffle(rng);
}
