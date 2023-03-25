use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a == b {
        println!("{}", a + b)
    } else {
        println!("{}", std::cmp::max(a, b) + std::cmp::max(a, b) - 1);
    }
}
