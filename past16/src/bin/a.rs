use proconio::*;

fn main() {
    input! {m: usize}
    println!("{}", if (4..=9).contains(&m) { "Yes" } else { "No" })
}
