use proconio::*;

fn main() {
    input! {r: usize}

    let s = r / 100;
    println!("{}", (s + 1) * 100 - r);
}
