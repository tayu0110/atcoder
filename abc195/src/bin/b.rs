use proconio::*;

fn main() {
    input! {a: usize, b: usize, w: usize}
    let w = w * 1000;

    let min = (w + b - 1) / b;
    let max = w / a;

    if min > max {
        println!("UNSATISFIABLE")
    } else {
        println!("{} {}", min, max)
    }
}
