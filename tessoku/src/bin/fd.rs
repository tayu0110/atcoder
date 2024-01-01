use proconio::*;

fn main() {
    input! {n: usize}

    println!("{n}");
    for i in 0..n {
        println!("{} {}", i + 1, (i + 1) % n + 1)
    }
}
