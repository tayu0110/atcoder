use std::collections::HashSet;

use proconio::*;

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let (x, y) = p[i];
            let (r, c) = p[j];
            let (dx, dy) = (x - r, y - c);
            if dx == 0 {
                set.insert((dx, dy.signum()));
            } else if dy == 0 {
                set.insert((dx.signum(), dy));
            } else {
                let g = gcd(dx.abs(), dy.abs());
                set.insert((dx / g, dy / g));
            }
        }
    }

    println!("{}", set.len());
}
