use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a % 2 == 0 {
        if b % 4 == 1 || b % 4 == 2 {
            println!("Angel")
        } else {
            println!("Devil")
        }
    } else {
        if b % 4 == 0 {
            println!("Devil")
        } else {
            println!("Angel")
        }
    }
}
