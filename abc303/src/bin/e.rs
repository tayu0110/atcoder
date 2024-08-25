use itertools::Itertools;
use proconio::*;

// 1: down edge, 2: upper edge, 4: core
fn rec(now: usize, par: usize, t: &Vec<Vec<usize>>, cores: &mut Vec<usize>) -> u8 {
    let mut f = true;
    let mut k = 0;
    for &to in &t[now] {
        if to == par {
            continue;
        }

        f = false;
        k |= rec(to, now, t, cores);
    }

    if f {
        return 1;
    }

    if k & 4 != 0 {
        2
    } else if k & 1 != 0 {
        cores.push(now);
        return 4;
    } else {
        return 1;
    }
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut cores = vec![];
    rec(0, 0, &t, &mut cores);

    let mut res = vec![];
    for c in cores {
        res.push(t[c].len());
    }

    res.sort();
    println!("{}", res.iter().join(" "))
}
