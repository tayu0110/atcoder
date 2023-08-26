use proconio::*;

fn main() {
    input! {n: char}

    println!(
        "{}",
        "3141592653589793238462643383279502884197169399375"
            .chars()
            .position(|v| v == n)
            .unwrap()
    )
}
