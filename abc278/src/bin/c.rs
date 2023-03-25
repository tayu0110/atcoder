#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {_n: usize, q: usize, p: [(usize, usize, usize); q]}

    let mut map = std::collections::HashMap::new();

    for (t, a, b) in p {
        if t == 1 {
            map.entry(a).or_insert(std::collections::HashSet::new()).insert(b);
        } else if t == 2 {
            map.entry(a).or_insert(std::collections::HashSet::new()).remove(&b);
        } else {
            if let Some(set_a) = map.get(&a) {
                if set_a.contains(&b) {
                    if let Some(set_b) = map.get(&b) {
                        if set_b.contains(&a) {
                            println!("Yes");
                            continue;
                        }
                    }
                }
            }

            println!("No");
        }
    }
}
