use proconio::*;

fn main() {
    input! {c: char, mut a: usize, d: char, mut b: usize}

    if c == 'E' {
        a += 180;
    } else {
        a = 180 - a;
    }
    if d == 'E' {
        b += 180;
    } else {
        b = 180 - b;
    }

    println!("{}", (a.max(b) - a.min(b)) / 15);
}
