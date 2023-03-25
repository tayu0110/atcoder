use proconio::*;

fn main() {
    input! {n: usize}

    println!(
        "{}",
        (1..=n)
            .map(|n| n.trailing_zeros())
            .max()
            .map(|v| 1 << v)
            .unwrap()
    )
}
