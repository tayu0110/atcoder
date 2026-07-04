use proconio::*;

fn main() {
    input! {s: String}
    println!("{}", s.replace("00", "0").len())
}
