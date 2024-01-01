use proconio::*;

fn main() {
    input! {a: usize, d: usize}

    println!("{}", (a * (d + 1)).max((a + 1) * d));
}
