use proconio::*;

fn main() {
    input! {s: [String; 12]}
    println!(
        "{}",
        s.into_iter()
            .enumerate()
            .filter(|(i, s)| i + 1 == s.len())
            .count()
    )
}
