use proconio::*;
use std::collections::HashMap;

fn main() {
    input! {q: usize}

    let mut map = HashMap::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: String, y: usize}
            map.insert(x, y);
        } else {
            input! {x: String}
            println!("{}", map.get(&x).unwrap())
        }
    }
}
