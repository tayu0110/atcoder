use proconio::*;

const M: usize = 998244353;

fn solve(now: usize, par: usize, s: &[String], t: &[Vec<usize>]) -> usize {
    if let Ok(r) = s[now].parse() {
        return r;
    }

    if s[now] == "+" {
        let mut res = 0;
        for &to in &t[now] {
            if to != par {
                res += solve(to, now, s, t);
                res %= M;
            }
        }
        res
    } else {
        let mut res = 1;
        for &to in &t[now] {
            if to != par {
                res *= solve(to, now, s, t);
                res %= M;
            }
        }
        res
    }
}

fn main() {
    input! {n: usize, p: [usize; n - 1], s: [String; n]}

    let mut t = vec![vec![]; n];
    for (i, p) in p.into_iter().enumerate() {
        t[p - 1].push(i + 1);
        t[i + 1].push(p - 1);
    }

    println!("{}", solve(0, 0, &s, &t))
}
