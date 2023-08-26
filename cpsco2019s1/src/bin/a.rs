use proconio::*;

fn main() {
    input! {n: usize, a: usize}

    println!(
        "{} {}",
        a / 3 + if a % 3 > 0 && (n - a) >= a % 3 { 1 } else { 0 },
        (n / 3).min(a)
    )
}
