use std::f64::consts::PI;

use cpio::scan;
use itertools::Itertools;

fn main() {
    scan!(n: usize, x1: i32, y1: i32, mut x2: i32, mut y2: i32, mut e: [(i32, i32); n]);

    let c1 = (x1 as f64 + x2 as f64) / 2.;
    let c2 = (y1 as f64 + y2 as f64) / 2.;

    let x2 = x2 as f64 - c1;
    let y2 = y2 as f64 - c2;
    let theta = 2. * PI - y2.atan2(x2);
    let cos = theta.cos();
    let sin = theta.sin();

    println!(
        "{}",
        e.into_iter()
            .map(|(a, b)| {
                let (a, b) = (a as f64 - c1, b as f64 - c2);
                let x = cos * a - sin * b;
                let y = sin * a + cos * b;
                format!("{x:.5} {y:.5}")
            })
            .join("\n")
    );
}
