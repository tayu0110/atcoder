use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let (a, b) = (a.min(b) as f64, a.max(b) as f64);

    if 3f64.sqrt() * b > a * 2f64 {
        println!("{}", 2f64 * a / 3f64.sqrt());
        return;
    }

    let theta = ((2f64 * a - 3f64.sqrt() * b) / b).atan();
    eprintln!("{}", theta);

    println!("{}", b / theta.cos());
}
