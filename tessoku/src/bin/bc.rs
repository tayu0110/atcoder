use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {q: usize}

    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {ty: usize}

        if ty == 1 {
            input! {x: i32}
            set.insert(x);
        } else if ty == 2 {
            input! {x: i32}
            set.remove(&x);
        } else {
            input! {x: i32}
            println!("{}", set.range(x..).next().unwrap_or(&-1))
        }
    }
}
