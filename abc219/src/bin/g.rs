use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, q: usize, e: [(usize, usize); m], x: [usize; q]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }
    let b = (m as f64).sqrt().round() as usize;
    let mut big = vec![vec![]; n];
    for i in 0..n {
        for &to in &t[i] {
            if t[to].len() >= b {
                big[i].push(to);
            }
        }
    }

    let mut res = (1..=n).map(|i| (i, -1)).collect::<Vec<_>>();
    let mut updated = vec![(0, -2); n];
    for (i, &x) in x.iter().enumerate() {
        for &to in &big[x - 1] {
            if updated[to].1 > res[x - 1].1 {
                res[x - 1] = updated[to];
            }
        }

        let lx = res[x - 1].0;
        if t[x - 1].len() < b {
            for &to in &t[x - 1] {
                res[to] = (lx, i as i32);
            }
        } else {
            updated[x - 1] = (lx, i as i32);
        }
    }

    for i in 0..n {
        for &to in &big[i] {
            if updated[to].1 > res[i].1 {
                res[i] = updated[to];
            }
        }
    }
    println!("{}", res.iter().map(|res| res.0).join(" "))
}
