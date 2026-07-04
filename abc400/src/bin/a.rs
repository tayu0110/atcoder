use proconio::*;

fn main() {
    input! {a: usize}
    if 400 % a == 0 {
        println!("{}", 400 / a);
    } else {
        println!("-1")
    }
}
