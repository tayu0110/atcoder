use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, d: usize}

    let mut dn = d * n;

    if dn > n * (n - 1) / 2 {
        println!("No");
        return;
    }

    println!("Yes");
    let mut set = HashSet::new();
    for i in 1.. {
        for j in 0..n {
            let k = (j + i) % n;

            if set.contains(&(j.min(k), j.max(k))) {
                continue;
            }

            set.insert((j.min(k), j.max(k)));
            println!("{} {}", j + 1, k + 1);
            dn -= 1;

            if dn == 0 {
                return;
            }
        }
    }
}
