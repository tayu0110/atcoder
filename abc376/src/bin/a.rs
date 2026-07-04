use proconio::*;

fn main() {
    input! {n: usize, c: usize, t: [usize; n]}

    let mut res = 1;
    let mut now = t[0];
    for i in 1..n {
        if t[i] - now >= c {
            now = t[i];
            res += 1;
        }
    }
    println!("{res}")
}
