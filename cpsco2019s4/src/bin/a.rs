use proconio::*;

fn main() {
    input! {l: usize, x: usize}

    if x / l % 2 == 0 {
        println!("{}", x % l)
    } else {
        println!("{}", l - x % l)
    }
}
