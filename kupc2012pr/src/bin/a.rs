use proconio::*;

fn main() {
    input! {m: u128, n: u32}

    if m == 0 {
        println!("{}", n + 1)
    } else if m == 1 {
        println!("{}", n + 2)
    } else if m == 2 {
        println!("{}", 2 * n + 3)
    } else {
        println!("{}", 2u128.pow(n + 3) - 3)
    }
}
