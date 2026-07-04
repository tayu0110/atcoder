use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}
    let m = s.iter().map(|s| s.len()).max().unwrap();

    for s in s {
        let k = (m - s.len()) / 2;
        println!("{}{}{}", ".".repeat(k), s, ".".repeat(k))
    }
}
