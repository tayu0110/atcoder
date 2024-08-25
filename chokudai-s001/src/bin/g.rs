use proconio::*;

fn main() {
    input! {n: usize, a: [marker::Chars; n]}
    println!(
        "{}",
        a.iter().fold(0, |s, v| {
            v.iter().fold(s, |s, v| {
                (s * 10 + (*v as usize - b'0' as usize)) % 1_000_000_007
            })
        })
    )
}
