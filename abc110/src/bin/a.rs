use proconio::*;

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();
    println!("{}", a[2] * 10 + a[1] + a[0])
}
