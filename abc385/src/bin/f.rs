use proconio::*;

fn main() {
    input! {n: usize, building: [(i64, i64); n]}

    let mut max = -1.0f64;
    for v in building.windows(2) {
        let (x1, h1) = v[0];
        let (x2, h2) = v[1];

        let num = h1 * x2 - h2 * x1;
        let den = x2 - x1;
        if num.signum() * den.signum() >= 0 {
            let b = num as f64 / den as f64;
            max = max.max(b);
        }
    }

    if max < 0.0 {
        println!("-1");
    } else {
        println!("{}", max)
    }
}
