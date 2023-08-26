use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n]}

    let mut v_score = HashMap::new();
    let mut h_score = HashMap::new();
    let mut point = HashMap::new();

    for (r, c, x) in p {
        point.insert((r, c), x);
        *v_score.entry(r).or_insert(0) += x;
        *h_score.entry(c).or_insert(0) += x;
    }

    let mut h_score = h_score.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    h_score.sort();
    h_score.reverse();

    let mut res = 0;
    for (k, v) in v_score {
        for &(hv, hk) in &h_score {
            let minus = *point.get(&(k, hk)).unwrap_or(&0);
            res = res.max(v + hv - minus);

            if !point.contains_key(&(k, hk)) {
                break;
            }
        }
    }

    println!("{}", res)
}
