use proconio::*;

fn main() {
    input! {m: usize, d: usize, y: usize, a: usize, b: usize}

    if b < d {
        println!("{} {} {}", y, a, b + 1)
    } else if a < m {
        println!("{} {} {}", y, a + 1, 1)
    } else {
        println!("{} {} {}", y + 1, 1, 1)
    }
}
