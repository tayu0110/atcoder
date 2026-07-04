use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n], q: usize}

    for _ in 0..q {
        input! {t: usize, d: usize}

        let (q, r) = p[t - 1];
        let rd = d % q;
        if rd == r {
            println!("{d}");
        } else if rd < r {
            println!("{}", d + (r - rd));
        } else {
            println!("{}", d + (q - rd) + r);
        }
    }
}
