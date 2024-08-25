use proconio::*;

fn main() {
    input! {n: usize, m: usize, h: [usize; n]}

    let mut res = 0;
    for i in 0..n {
        res += h[i];
        if res > m {
            println!("{}", i);
            return;
        }
    }

    println!("{n}");
}
