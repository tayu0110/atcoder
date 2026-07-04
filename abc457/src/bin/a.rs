use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], x: usize}
    println!("{}", a[x - 1])
}
