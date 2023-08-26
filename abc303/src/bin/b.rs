use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [[usize; n]; m]}

    let mut set = HashSet::new();
    for v in a {
        for v in v.windows(2) {
            set.insert((v[0] - 1, v[1] - 1));
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            if !set.contains(&(i, j)) && !set.contains(&(j, i)) {
                res += 1;
            }
        }
    }

    println!("{}", res)
}
