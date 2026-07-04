use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let res = a as f64 / b as f64;
    let mut diff = f64::MAX;
    let mut t = 0;
    for i in 0..10000 {
        if diff > (res - i as f64).abs() {
            diff = (res - i as f64).abs();
            t = i;
        }
    }

    println!("{t}")
}
