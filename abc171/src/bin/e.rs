use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let xor = a.iter().fold(0, |s, &v| s ^ v);

    println!("{}", a.into_iter().map(|v| v ^ xor).join(" "))
}
