use proconio::*;

fn main() {
    input! {x: usize}

    println!("{}", x / 500 * 1000 + (x % 500) / 5 * 5);
}
