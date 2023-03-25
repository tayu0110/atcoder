use proconio::*;

fn main() {
    input! {n: usize, mut x: [usize; 5*n]}
    x.sort();

    let t = x.into_iter().skip(n).take(3 * n).sum::<usize>();
    println!("{}", t as f64 / (3 * n) as f64);
}
