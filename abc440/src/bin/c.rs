use std::fmt::Write as _;

use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, w: usize, c: [i64; n]}

        let mut ret = 0;
        let mut t = vec![0; w];
        for i in 0..n {
            let rem = (i + 1) % (2 * w);
            if rem < w {
                ret += c[i];
                t[rem] -= c[i];
            } else {
                t[rem - w] += c[i];
            }
        }

        let mut base = ret;
        for i in 0..w {
            base += t[i];
            ret = ret.min(base);
        }
        for i in 0..w {
            base -= t[i];
            ret = ret.min(base);
        }

        writeln!(buf, "{ret}").unwrap();
    }

    print!("{buf}")
}
