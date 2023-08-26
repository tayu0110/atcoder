use proconio::*;

fn main() {
    input! {n: usize, k: usize, m: usize, a: [usize; n-1]}
    let res = (m * n).saturating_sub(a.iter().sum());
    if res > k {
        println!("-1")
    } else {
        println!("{}", res)
    }
}
