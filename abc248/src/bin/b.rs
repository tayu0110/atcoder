use proconio::*;

fn main() {
    input! {mut a: usize, b: usize, k: usize}

    for i in 0.. {
        if a >= b {
            println!("{i}");
            return;
        }
        a *= k;
    }
}
