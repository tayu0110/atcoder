use proconio::*;

fn main() {
    input! {x: u128};

    let mut now = 100;
    for i in 0.. {
        if now >= x {
            println!("{i}");
            return;
        }
        now = now * 101 / 100;
    }
}
