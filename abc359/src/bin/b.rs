use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 2 * n]}
    println!("{}", a.windows(3).filter(|v| v[0] == v[2]).count())
}
