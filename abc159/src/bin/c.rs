use proconio::*;

fn main() {
    input! {l: usize}

    let a = l as f64 / 3.0;
    println!("{}", a * a * a)
}
