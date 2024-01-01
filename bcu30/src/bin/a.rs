use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; k]}

    let mut now = 0;
    for a in a {
        if now + a == n {
            println!("{n}");
            return;
        } else if now + a < n {
            now += a;
        } else {
            now = n - (a - (n - now));
        }
    }

    println!("{now}")
}
