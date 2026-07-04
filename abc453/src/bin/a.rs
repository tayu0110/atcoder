use proconio::*;

fn main() {
    input! {_: usize, s: String}
    let s = s.trim_start_matches('o');
    println!("{s}")
}
