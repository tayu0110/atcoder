use proconio::*;

fn main() {
    input! {n: usize, k: usize, t: [usize; n]}

    for (i, v) in t.windows(3).enumerate() {
        if v.iter().sum::<usize>() < k {
            println!("{}", i + 3);
            return;
        }
    }

    println!("-1")
}
