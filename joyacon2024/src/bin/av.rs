use proconio::*;

fn main() {
    input! {k: usize}

    let odd = (k + 1) / 2;
    let even = k / 2;
    println!("{}", odd * even)
}
