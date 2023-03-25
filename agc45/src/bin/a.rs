use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {t: usize};

    for _ in 0..t {
        input! {n: usize, a: [usize; n], s: Bytes};

        let mut b = vec![];
        let mut good = false;
        for i in (0..n).rev() {
            let mut x = a[i];
            for &y in &b {
                x = std::cmp::min(x, x ^ y);
            }
            if x != 0 {
                if s[i] == b'1' {
                    good = true;
                }
                b.push(x);
            }
        }

        if good {
            println!("1");
        } else {
            println!("0");
        }
    }
}
