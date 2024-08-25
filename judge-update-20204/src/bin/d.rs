use std::collections::BTreeMap;

use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    input! {n: usize, q: usize, a: [usize; n], s: [usize; q]}

    let mut b = a.clone();
    for i in 1..n {
        b[i] = gcd(b[i], b[i - 1]);
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        map.entry(b[i]).or_insert(i);
    }

    'b: for s in s {
        let mut x = s;
        for (k, v) in map.iter().rev() {
            x = gcd(x, *k);

            if x == 1 {
                println!("{}", v + 1);
                continue 'b;
            }
        }

        println!("{x}")
    }
}
