use proconio::input;

fn dfs(now: usize, par: usize, thr: i64, a: &Vec<i64>, t: &Vec<Vec<usize>>) -> i64 {
    let mut res = 0;

    for to in &t[now] {
        if &par == to {
            continue;
        }

        res += dfs(*to, now, thr, a, t);
    }

    std::cmp::max(0, res - 1) +
        if a[now] >= thr {
            1
        } else {
            0
        }
}

fn main() {
    input! {n: usize, mut a: [i64; n-1], p: [(usize, usize); n-1]}
    a.insert(0, 0);

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let (mut l, mut r) = (-1, std::i64::MAX >> 8);
    while r - l > 1 {
        let m = (r + l) / 2;

        if dfs(0, std::usize::MAX, m, &a, &t) >= 1 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}