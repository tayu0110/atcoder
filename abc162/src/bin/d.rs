use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}

    let mut t = vec![vec![]; 2];
    let mut set = HashSet::new();
    for (i, c) in s.into_iter().enumerate() {
        if c == 'R' {
            t[0].push(i);
        } else if c == 'G' {
            t[1].push(i);
        } else {
            set.insert(i);
        }
    }

    let mut res = 0;
    for &i in &t[0] {
        for &j in &t[1] {
            let (i, j) = (i.min(j), i.max(j));

            let mut t = set.len();
            if set.contains(&(j + (j - i))) {
                t -= 1;
            }
            if i >= j - i && set.contains(&(i - (j - i))) {
                t -= 1;
            }
            if (i + j) % 2 == 0 && set.contains(&((i + j) / 2)) {
                t -= 1;
            }

            res += t;
        }
    }

    println!("{}", res)
}
