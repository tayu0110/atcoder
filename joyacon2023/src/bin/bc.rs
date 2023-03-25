use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, q: usize, s: Chars}

    let mut top = 0;
    for _ in 0..q {
        input! {t: usize, x: usize}

        if t == 1 {
            top = (top + n - x) % n;
        } else {
            println!("{}", s[(top + x - 1) % n]);
        }
    }
}
