use proconio::*;
use rational::Rational;
use std::collections::{BTreeSet, HashSet};

fn main() {
    input! {n: usize, k: usize, p: [(i64, i64); n]}

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut res = 0;
    // let mut set = BTreeSet::new();
    let mut nan = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x, y) = p[i];
            let (nx, ny) = p[j];
            let a = Rational::<i64>::new(ny - y, nx - x);
            if a.is_nan() {
                if nan.contains(&x) {
                    continue;
                }
                nan.insert(x);
                if p.iter().map(|p| p.0).filter(|px| *px == x).count() >= k {
                    res += 1;
                }
                continue;
            }
            let b = Rational::<i64>::new(y, 1) - a * Rational::<i64>::new(x, 1);
            // if set.contains(&(a, b)) {
            //     continue;
            // }
            // set.insert((a, b));

            let mut cnt = 2;
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }

                let (px, py) = p[k];
                let ry = a * Rational::<i64>::new(px, 1) + b;
                if ry == Rational::<i64>::new(py, 1) {
                    cnt += 1;
                }
            }

            if cnt >= k {
                res += 1;
            }
        }
    }

    println!("{res}");
}
