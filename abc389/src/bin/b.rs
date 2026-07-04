use proconio::*;

fn main() {
    input! {x: usize}
    let mut now = 1;
    for i in 1.. {
        now *= i;
        if now == x {
            println!("{i}");
            return;
        }
    }
}
