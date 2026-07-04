use proconio::*;

fn main() {
    input! {x: usize, c: usize}

    println!("{}", (x / (c + 1000) * 1000))
}
