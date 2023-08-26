use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a >= 13 {
        println!("{}", b);
    } else if 6 <= a {
        println!("{}", b / 2)
    } else {
        println!("0")
    }
}
