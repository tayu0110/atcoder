use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars, q: usize, query: [(usize, usize, char); q]}

    let mut lower = HashSet::new();
    let mut upper = HashSet::new();
    for (i, &c) in s.iter().enumerate() {
        if c.is_lowercase() {
            lower.insert(i);
        } else {
            upper.insert(i);
        }
    }

    for (t, x, c) in query {
        if t == 1 {
            s[x - 1] = c;
            if c.is_lowercase() && upper.contains(&(x - 1)) {
                upper.remove(&(x - 1));
                lower.insert(x - 1);
            } else if c.is_uppercase() && lower.contains(&(x - 1)) {
                lower.remove(&(x - 1));
                upper.insert(x - 1);
            }
        } else if t == 2 {
            if upper.is_empty() {
                continue;
            }
            if lower.is_empty() {
                std::mem::swap(&mut lower, &mut upper);
                continue;
            }

            if lower.len() < upper.len() {
                std::mem::swap(&mut lower, &mut upper);
            }
            lower.extend(upper.iter());
            upper.clear();
        } else {
            if lower.is_empty() {
                continue;
            }
            if upper.is_empty() {
                std::mem::swap(&mut lower, &mut upper);
                continue;
            }

            if lower.len() > upper.len() {
                std::mem::swap(&mut lower, &mut upper);
            }
            upper.extend(lower.iter());
            lower.clear();
        }
    }

    let mut res = vec![];
    for i in 0..n {
        let c = s[i];
        if upper.contains(&i) {
            res.push(c.to_uppercase().next().unwrap());
        } else {
            res.push(c.to_lowercase().next().unwrap());
        }
    }

    println!("{}", res.iter().join(""))
}
