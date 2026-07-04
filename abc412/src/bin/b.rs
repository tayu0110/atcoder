use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let t = t.into_iter().collect::<HashSet<_>>();
    for v in s.windows(2) {
        if v[1].is_ascii_uppercase() && !t.contains(&v[0]) {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
