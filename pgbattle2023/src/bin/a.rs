use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let res = a.into_iter().map(|a| a.signum()).fold(1, |s, v| s * v);
    if res > 0 {
        println!("+")
    } else if res == 0 {
        println!("0")
    } else {
        println!("-")
    }
}
