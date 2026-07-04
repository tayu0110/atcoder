use proconio::*;

fn main() {
    input! {s: String}
    println!(
        "{}",
        s.chars()
            .filter(|c| c.is_ascii_uppercase())
            .collect::<String>()
    )
}
