use proconio::*;

fn main() {
    input! {n: usize, a: u8, b: u8}
    println!(
        "{}",
        (1..=n)
            .filter(|v| {
                let s = v.to_string().chars().fold(0, |s, v| s + v as u8 - b'0');
                a <= s && s <= b
            })
            .sum::<usize>()
    )
}
