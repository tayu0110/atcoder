use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, c: [usize; n]}

    let res = a + b;
    println!("{}", c.into_iter().position(|c| c == res).unwrap() + 1)
}
