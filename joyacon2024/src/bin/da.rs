use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    println!("{}", a.saturating_sub(b * 2))
}
