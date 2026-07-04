use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, w: [usize; n], e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut ret = vec![usize::MAX; n];
    let mut min_w = vec![usize::MAX; n];
    let mut stack = vec![vec![]; n];
    ret[0] = 0;
    min_w[0] = 0;
    stack[0].push((0, 0));
    loop {
        let mut update = false;
        for i in 0..n {
            while let Some((nw, x)) = stack[i].pop() {
                let next_w = nw + w[i];
                let next_x = x + next_w;
                for &to in &t[i] {
                    if next_x < ret[to] {
                        ret[to] = ret[to].min(next_x);
                        min_w[to] = min_w[to].min(next_w);
                        stack[to].push((next_w, next_x));
                        update = true;
                    }
                }
            }
        }

        if !update {
            break;
        }
    }

    println!("{}", ret.iter().join("\n"))
}
