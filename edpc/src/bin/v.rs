use proconio::*;

fn rec(now: usize, par: usize, m: usize, memo: &mut Vec<Vec<usize>>, t: &Vec<Vec<usize>>) -> usize {
    let children = t[now].len();
    let mut res = 1;
    for i in 0..children {
        if t[now][i] == par {
            continue;
        }

        memo[now][i] = rec(t[now][i], now, m, memo, t);
        res *= memo[now][i];
        res %= m;
    }

    (res + 1) % m
}

fn rec2(
    now: usize,
    par: usize,
    r: usize,
    m: usize,
    memo: &mut Vec<Vec<usize>>,
    t: &Vec<Vec<usize>>,
) {
    let len = t[now].len();

    let f = {
        let mut cum = 1;
        let mut buf = vec![];
        for i in 0..len {
            if memo[now][i] == std::usize::MAX {
                memo[now][i] = r;
            }
            cum *= memo[now][i];
            cum %= m;
            buf.push(cum);
        }
        buf
    };
    let b = {
        let mut cum = 1;
        let mut buf = vec![];
        for i in (0..len).rev() {
            cum *= memo[now][i];
            cum %= m;
            buf.push(cum);
        }
        buf.reverse();
        buf
    };

    for i in 0..len {
        if t[now][i] == par {
            continue;
        }
        let mut r = 1;
        if i > 0 {
            r *= f[i - 1];
            r %= m;
        }
        if i + 1 < len {
            r *= b[i + 1];
            r %= m;
        }
        rec2(t[now][i], now, (r + 1) % m, m, memo, t);
    }
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    let mut memo = vec![vec![]; n];
    for (x, y) in e.into_iter().map(|(x, y)| (x - 1, y - 1)) {
        t[x].push(y);
        t[y].push(x);
        memo[x].push(std::usize::MAX);
        memo[y].push(std::usize::MAX);
    }

    rec(0, 0, m, &mut memo, &t);
    rec2(0, 0, 0, m, &mut memo, &t);

    for i in 0..n {
        let res = memo[i].iter().fold(1, |s, v| (s * v) % m);
        println!("{}", res);
    }
}
