use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let min = a.iter().flatten().min().unwrap();
    println!("{}", a.iter().flatten().map(|v| v - min).sum::<usize>())
}
