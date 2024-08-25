use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let min = (m * 6) as f64;
    let hour = 30f64 * (n % 12) as f64 + m as f64 * 0.5;

    let d = hour.max(min) - hour.min(min);

    println!("{}", d.min(360.0 - d))
}
