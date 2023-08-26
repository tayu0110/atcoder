use proconio::*;

fn main() {
    input! {k: usize, n: usize, mut a: [usize; n]}
    a.push(a[0] + k);
    println!("{}", k - a.windows(2).map(|v| v[1] - v[0]).max().unwrap())
}
