use std::f64::consts::PI;

use proconio::*;

fn main() {
    input! {h: f64, w: f64, d: f64}

    if h * h + w * w <= d * d * 4. {
        println!("1");
        return;
    }
    let mut res = d * d * PI;
    for w in [h, w] {
        if d * 2. > w {
            let theta = (w / 2. / d).acos();
            let h = theta.sin() * d;
            let s = d * d * theta * 2. - w * h;
            res -= s;
        }
    }

    println!("{}", res / (h * w))
}
