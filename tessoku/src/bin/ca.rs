use proconio::*;

fn main() {
    input! {a: u32, b: u32}
    println!(
        "{}",
        (a..=b).find(|&a| 100 % a == 0)
            .and(Some("Yes"))
            .unwrap_or("No")
    )
}
