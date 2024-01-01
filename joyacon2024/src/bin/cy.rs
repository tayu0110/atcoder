use proconio::*;

fn main() {
    input! {n: usize, k: usize}

    if k == 1 {
        println!("0");
        return;
    }

    println!("{}", n - (k - 1) - 1);
}
