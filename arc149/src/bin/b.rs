#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn lis(a: &Vec<usize>) -> usize {
    let n = a.len();
    let mut dp = vec![std::usize::MAX; n];

    for v in a {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if dp[m as usize] < *v {
                l = m;
            } else {
                r = m;
            }
        }

        dp[r as usize] = *v;
    }

    dp.into_iter().take_while(|v| *v < std::usize::MAX).count()
}

fn solve(c: &[(usize, usize)]) -> usize {
    let (a, b) = c.iter().map(|(a, b)| (*a, *b)).unzip::<usize, usize, Vec<usize>, Vec<usize>>();

    lis(&a) + lis(&b)
}

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut c = a.into_iter().zip(b).collect_vec();

    c.sort();
    let mut res = solve(&c);

    c.sort_by_key(|(_, r)| *r);
    res = std::cmp::max(res, solve(&c));

    println!("{}", res);
}
