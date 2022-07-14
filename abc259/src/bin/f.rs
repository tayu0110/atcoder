#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

const INF: i64 = 0x3f3f3f3f3f3f3f3f;

fn rec(now: usize, par: usize, par_link: usize, d: &Vec<usize>, ck: &mut Vec<bool>, t: &Vec<Vec<(usize, i64)>>, memo: &mut Vec<Vec<i64>>) -> i64 {
    if memo[par_link][now] != -INF {
        return memo[par_link][now];
    }
    ck[now] = true;
    let nd = d[now] - par_link;
    let mut buf = vec![];
    let mut res = 0;

    for (to, w) in &t[now] {
        if to == &par {
            continue;
        }
        let r = rec(*to, now, 0, d, ck, t, memo);
        res += r;
        let r2 = rec(*to, now, 1, d, ck, t, memo) + w;
        buf.push(std::cmp::Reverse(r2 - r));
    }

    buf.sort();
    for (i, std::cmp::Reverse(diff)) in buf.into_iter().enumerate() {
        if i == nd {
            break;
        }
        if diff < 0 {
            continue;
        }
        res += diff;
    }

    memo[par_link][now] = res;
    res
}

fn main() {
	input! {n: usize, d: [usize; n], p: [(usize, usize, i64); n-1]};

    let mut t = vec![vec![]; n];
    for (u, v, w) in p {
        if d[u-1] == 0 || d[v-1] == 0 {
            continue;
        }
        if w < 0 {
            continue;
        }
        t[u-1].push((v-1, w));
        t[v-1].push((u-1, w));
    }

    let mut memo = vec![vec![-INF; n]; 2];
    let mut ck = vec![false; n];
    let mut res = 0;

    for i in 0..n {
        if ck[i] {
            continue;
        }
        res += rec(i, INF as usize, 0, &d, &mut ck, &t, &mut memo);
    }

    println!("{}", res);
}
