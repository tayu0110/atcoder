use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize}

    let mut map = HashMap::new();
    for i in 1..n {
        for j in (1..n).take_while(|j| i * *j <= n) {
            *map.entry(i * j).or_insert(0) += 1;
        }
    }

    let mut res = 0;
    for (&k, &v) in &map {
        if let Some(nv) = map.get(&(n - k)) {
            res += v * *nv;
        }
    }

    println!("{}", res)
}
