use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, p: [(usize, usize); n], s: Chars}

    let mut map = std::collections::HashMap::new();
    for (i, (x, y)) in p.into_iter().enumerate() {
        map.entry(y).or_insert(vec![]).push((x, s[i]));
    }

    for (_, v) in map.iter_mut() {
        v.sort();

        let mut f = false;
        for (_, c) in v {
            if *c == 'R' {
                f = true;
            } else {
                if f {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
