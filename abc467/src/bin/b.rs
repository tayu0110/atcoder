use proconio::*;

fn main() {
    input! {n: usize}
    let mut x = 10000;
    let mut y = 10000;
    for _ in 0..n {
        input! {a: usize, b: usize, s: String}

        x -= a;
        if s == "take" {
            y -= a;
        } else {
            y -= b;
        }
    }

    println!("{}", x - y);
}
