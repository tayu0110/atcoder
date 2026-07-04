use proconio::*;

fn main() {
    input! {s: String}
    println!("{}", &format!("#{s}").replace("#.", "#o")[1..]);
}
