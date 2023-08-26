use proconio::*;

fn main() {
    input! {n: usize, h: usize, x: usize, p: [usize; n]}

    for i in 0..n {
        if h + p[i] >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
