#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: f64, y: f64, r: f64}
    const GETA: i64 = 10000;

    let x = (x * GETA as f64).round() as i64;
    let y = (y * GETA as f64).round() as i64;
    let r = (r * GETA as f64).round() as i64;

    let ymin = if y - r >= 0 {
        (y - r + GETA - 1) / GETA * GETA
    } else {
        (y - r) / GETA * GETA
    };

    let mut res = 0;
    let inner_circle = |nx: i64, ny: i64| (x - nx) * (x - nx) + (y - ny) * (y - ny) <= r * r;
    for ny in (ymin..=y+r).step_by(GETA as usize) {
        let xmax = {
            let (mut l, mut r) = (x, x + r + 10000000);
            while r - l > 1 {
                let m = (r + l) / 2;
                if inner_circle(m, ny) {
                    l = m;
                } else {
                    r = m;
                }
            }
            if l >= 0 {
                l / GETA
            } else {
                -((l.abs() + GETA - 1) / GETA)
            }
        };
        let xmin = {
            let (mut l, mut r) = (x-r - 10000000, x);
            while r - l > 1 {
                let m = (r + l) / 2;
                if !inner_circle(m, ny) {
                    l = m;
                } else {
                    r = m;
                }
            }
            if l >= 0 {
                l / GETA
            } else {
                -((l.abs() + GETA - 1) / GETA)
            }
        };
        res += xmax - xmin;
    }

    println!("{}", res);
}
