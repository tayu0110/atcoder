use proconio::*;

fn main() {
    input! {x: usize}

    let mut now = 100;
    for i in 1.. {
        now = (now as u128 * 101 / 100) as usize;
        if now >= x {
            println!("{}", i);
            return;
        }
    }
}
