#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn solve(target: usize, a: usize, p: &[(usize, usize, usize)]) -> usize {
    let (w, x, v) = p[target];
    
    let mut score = w;
    let mut event = vec![];
    let mut set = std::collections::HashSet::new();
    for (i, &(nw, nx, nv)) in p.iter().enumerate() {
        if i == target {
            continue;
        }

        if nx < x && v < nv {
            event.push(((x - nx) as f64 / (nv - v) as f64, i));
            event.push(((x+a - nx) as f64 / (nv - v) as f64, i));
        } else if x+a < nx && nv < v {
            event.push(((nx - (x+a)) as f64 / (v - nv) as f64, i));
            event.push(((nx - x) as f64 / (v - nv) as f64, i));
        } else if x <= nx && nx <= x+a {
            if v < nv {
                event.push(((x+a - nx) as f64 / (nv - v) as f64, i));
            } else if v > nv {
                event.push(((nx - x) as f64 / (v - nv) as f64, i));
            }
            score += nw;
            set.insert(i);
        }
    }

    event.sort_by(|&(t, _), &(nt, _)| t.partial_cmp(&nt).unwrap());

    let mut res = score;
    for (_, i) in event {
        let (w, _, _) = p[i];
        if set.contains(&i) {
            score -= w;
            set.remove(&i);
        } else {
            set.insert(i);
            score += w;
        }

        res = std::cmp::max(res, score);
    }

    res
}

#[fastout]
fn main() {
    input! {n: usize, a: usize, p: [(usize, usize, usize); n]}

    let mut res = 0;
    for i in 0..n {
        let r = solve(i, a, &p);
        if res < r {
            res = r;
        }
    }

    println!("{}", res);
}
