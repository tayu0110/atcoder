use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {q: usize}

    let mut map = HashMap::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: usize}
            *map.entry(x).or_insert(0) += 1;
        } else if ty == 2 {
            input! {x: usize}
            *map.entry(x).or_insert(0) -= 1;
            if *map.get(&x).unwrap() == 0 {
                map.remove(&x);
            }
        } else {
            println!("{}", map.len())
        }
    }
}
