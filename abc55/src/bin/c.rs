use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    if m < n * 2 {
        println!("{}", m / 2);
    } else {
        println!("{}", n + (m - 2 * n) / 4)
    }
}
