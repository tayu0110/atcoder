use proconio::*;

fn main() {
    input! {n: usize}
    let rem = n % 1000;
    if rem == 0 {
        println!("0")
    } else {
        println!("{}", 1000 - rem)
    }
}
