use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, res: &mut [u32], t: &Vec<Vec<usize>>) -> u32 {
    let mut r = 1;
    for &to in &t[now] {
        r += dfs(to, res, t);
    }
    res[now] = r;
    r
}

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out);
    input! { n: usize, p: [usize; n-1] }

    let mut t = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        t[p - 1].push(i + 1);
    }

    let mut res = vec![0; n];
    dfs(0, &mut res, &t);
    writeln!(out, "{}", res.iter().join("\n")).ok();
}
