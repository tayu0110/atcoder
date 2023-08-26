use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}
    let max = p.iter().max().unwrap();
    println!("{}", p.iter().position(|v| v == max).unwrap() + 1)
}
