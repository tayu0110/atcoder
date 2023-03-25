use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a % 2 == 0 && b % 2 == 1 {
        println!("{}", (b + 1 - a) / 2 % 2)
    } else if a % 2 == 0 && b % 2 == 0 {
        println!("{}", ((b + 2 - a) / 2 % 2) ^ (b + 1))
    } else if a % 2 == 1 && b % 2 == 1 {
        println!("{}", ((b + 1 - (a - 1)) / 2 % 2) ^ (a - 1))
    } else {
        println!("{}", ((b + 2 - (a - 1)) / 2 % 2) ^ (b + 1) ^ (a - 1))
    }
}
