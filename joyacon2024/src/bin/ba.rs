use proconio::*;

fn main() {
    input! {n: usize, d: usize}

    let d = 2 * d + 1;
    println!("{}", (n + d - 1) / d);
}
