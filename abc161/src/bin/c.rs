use proconio::*;

fn main() {
    input! {n: usize, k: usize}
    let nn = n % k;
    println!("{}", nn.min(k - nn))
}
