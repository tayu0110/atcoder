use itertools::Itertools;
use proconio::*;
use rand::{thread_rng, Rng, SeedableRng};
use std::{cmp::Ordering, io::Write, time::SystemTime};

fn gen(seed: u64, n: usize, d: usize) -> Vec<i64> {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed ^ 94);
    let mut ws = vec![];
    let exp = rand_distr::Exp::new(1e-5).unwrap();
    for _ in 0..n {
        loop {
            let w = rng.sample(exp);
            if w <= n as f64 * 1e5 / d as f64 {
                ws.push((f64::round(w) as i64).max(1));
                break;
            }
        }
    }
    ws
}

fn main() {
    let now = SystemTime::now();
    input_interactive!(n: usize, d: usize, mut q: usize);

    let mut rng = thread_rng();
    let mut t = vec![vec![]; d];
    for i in 0..n {
        if i < d {
            t[i].push(i);
            continue;
        }

        let g = rng.gen_range(0..d);
        t[g].push(i);
    }

    let mut cmp = vec![];
    let nq = (q / 3).max(60);
    q -= nq;
    for _ in 0..nq {
        let l = rng.gen_range(0..n);
        let mut r = rng.gen_range(0..n);
        while r == l {
            r = rng.gen_range(0..n);
        }

        println!("{} {} {} {}", 1, 1, l, r);
        std::io::stdout().flush().ok();
        input_interactive!(c: char);

        cmp.push((
            l,
            r,
            if c == '<' {
                Ordering::Less
            } else if c == '=' {
                Ordering::Equal
            } else {
                Ordering::Greater
            },
        ));
    }
    let mut cmp_line = vec![];
    for _ in 0..q {
        let l = rng.gen_range(0..d);
        let mut r = rng.gen_range(0..d);
        while r == l {
            r = rng.gen_range(0..d);
        }

        println!(
            "{} {} {} {}",
            t[l].len(),
            t[r].len(),
            t[l].iter().join(" "),
            t[r].iter().join(" ")
        );
        std::io::stdout().flush().ok();
        input_interactive!(c: char);

        cmp_line.push((
            l,
            r,
            if c == '<' {
                Ordering::Less
            } else if c == '=' {
                Ordering::Equal
            } else {
                Ordering::Greater
            },
        ));
    }

    let mut max_unmatch_w = i32::MAX;
    let mut max_unmatch_line = i32::MAX;
    let mut maybe_match = vec![];
    while now.elapsed().unwrap().as_millis() < 1500 {
        let seed: u64 = rng.gen();
        let w = gen(seed, n, d);
        let t_sum = t
            .iter()
            .map(|v| v.iter().map(|v| w[*v]).sum::<i64>())
            .collect::<Vec<_>>();

        let mut unmatch_w = 0;
        for &(l, r, c) in &cmp {
            if w[l].cmp(&w[r]) != c {
                unmatch_w += 1;
            }
        }
        let mut unmatch_line = 0;
        for &(l, r, c) in &cmp_line {
            if t_sum[l].cmp(&t_sum[r]) != c {
                unmatch_line += 1;
            }
        }

        if max_unmatch_w > unmatch_w {
            maybe_match.clear();
            maybe_match.push(w);
            max_unmatch_w = unmatch_w;
            max_unmatch_line = unmatch_line;
        } else if max_unmatch_w == unmatch_w {
            if max_unmatch_line > unmatch_line {
                maybe_match.clear();
                maybe_match.push(w);
                max_unmatch_w = unmatch_w;
                max_unmatch_line = unmatch_line;
            } else if max_unmatch_line == unmatch_line {
                maybe_match.push(w);
            }
        }
    }

    let w = maybe_match.pop().unwrap();
    let mut ws = w
        .into_iter()
        .enumerate()
        .map(|(l, r)| (r, l))
        .collect::<Vec<_>>();
    ws.sort();
    let mut res = vec![0; n];
    let mut sum = vec![0; d];
    while let Some((w, index)) = ws.pop() {
        let mut tar = 0;
        let mut min = i64::MAX;
        for i in 0..d {
            if min > sum[i] {
                min = sum[i];
                tar = i;
            }
        }

        res[index] = tar;
        sum[tar] += w;
    }

    println!("{}", res.iter().join(" "));
    std::io::stdout().flush().ok();
}
