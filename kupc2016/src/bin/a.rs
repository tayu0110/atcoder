use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, t: [usize; n]}
    println!("{}", t.into_iter().filter(|&v| v < a || b <= v).count())
}
