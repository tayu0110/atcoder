use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    let mut res = b / 4 - (a - 1) / 4;
    res -= b / 100 - (a - 1) / 100;
    res += b / 400 - (a - 1) / 400;
    println!("{}", res)
}
