use proconio::*;

fn main() {
    input! {n: usize, m: usize}
    let mut now = m;
    for i in 1.. {
        now = n % now;
        if now == 0 {
            println!("{i}");
            return;
        }
    }
}
