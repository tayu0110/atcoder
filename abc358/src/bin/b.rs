use proconio::*;

fn main() {
    input! {n: usize, a: usize, t: [usize; n]}

    let mut now = 0;
    for t in t {
        if now < t {
            now = t;
        }
        now += a;
        println!("{now}");
    }
}
