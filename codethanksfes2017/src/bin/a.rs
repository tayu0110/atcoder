use proconio::*;

fn main() {
    input! {t: [usize; 8]}
    println!("{}", t.iter().max().unwrap())
}
