#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn solve(t: usize, a: &[usize]) -> Vec<usize> {
    let n = a.len();

    let mut res = vec![];
    for i in 0..(1 << n) {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += a[j];
            }
        }

        if sum <= t {
            res.push(sum);
        }
    }

    res
}

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}

    if n < 15 {
        println!("{}", solve(t, &a).iter().max().unwrap());
        std::process::exit(0);
    }

    let mut b = solve(t, &a[n/2..]);
    let a = solve(t, &a[0..n/2]);

    b.sort();

    let mut res = 0;
    for v in a {
        let (mut l, mut r) = (0, b.len());

        while r - l > 1 {
            let m = (r + l) / 2;
            if v + b[m] <= t {
                l = m;
            } else {
                r = m;
            }
        }

        if v + b[l] <= t {
            res = std::cmp::max(res, v+b[l]);
        }
    }

    println!("{}", res);
}
