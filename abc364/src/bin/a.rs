use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    if s[..n - 1]
        .windows(2)
        .any(|v| v.iter().all(|s| s == "sweet"))
    {
        println!("No")
    } else {
        println!("Yes")
    }
}
