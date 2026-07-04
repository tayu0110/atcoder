use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut a = (1..=n).collect::<Vec<_>>();
    let mut base = 0;
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {p: usize, x: usize}
            a[(base + p - 1) % n] = x;
        } else if ty == 2 {
            input! {p: usize}
            println!("{}", a[(base + p - 1) % n]);
        } else {
            input! {k: usize}
            base += k;
            base %= n;
        }
    }
}
