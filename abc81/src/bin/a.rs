use proconio::*;

fn main() {
    input! {s: String}
    let s = u32::from_str_radix(&s, 2).unwrap();
    println!("{}", s.count_ones())
}
