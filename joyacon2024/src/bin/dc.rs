use proconio::*;

fn main() {
    input! {mut a: usize, b: usize, mut c: usize, d: usize}

    while a > 0 && c > 0 {
        if c <= b {
            println!("Yes");
            return;
        }

        c -= b;

        if a <= d {
            println!("No");
            return;
        }

        a -= d;
    }
}
