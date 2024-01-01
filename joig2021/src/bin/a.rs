use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}
    let max = a.max(b).max(c);
    println!("{}", max * 3 - a - b - c)
}
