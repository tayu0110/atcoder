use proconio::*;

fn main() {
    input! {x: usize, n: usize}
    let mut now = x;
    for i in 1.. {
        if now % 3 == 0 {
            now += 1;
        } else if now % 3 == 1 {
            now *= 2;
        } else {
            now *= 3;
        }

        if now >= n {
            println!("{i}");
            return;
        }
    }
}
