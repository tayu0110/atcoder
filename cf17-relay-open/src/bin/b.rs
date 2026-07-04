use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    for _ in 0..q {
        input! {mut v: usize, mut w: usize}

        if n == 1 {
            println!("{}", v.min(w));
        } else {
            while v != w {
                if v < w {
                    w = (w + n - 2) / n;
                } else {
                    v = (v + n - 2) / n;
                }
            }

            println!("{v}")
        }
    }
}
