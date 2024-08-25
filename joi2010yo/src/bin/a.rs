use proconio::*;

fn main() {
    input! {a: [usize; 10]}
    println!("{}", a.into_iter().reduce(|s, v| s - v).unwrap())
}
