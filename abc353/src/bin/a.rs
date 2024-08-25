use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    for i in 0..n {
        if h[i] > h[0] {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1")
}
