#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [usize; n]};

    let s = a.iter().fold(0, |x, v| x ^ *v);
    a.iter_mut().for_each(|v| *v &= !s);

    let mut r = 0;
    for i in (0..61).rev() {
        for j in r..n {
            if (a[j] >> i) & 1 != 0 {
                a.swap(j, r);
            }
        }

        if (a[r] >> i) & 1 == 0 {
            continue;
        }

        let now = a[r];
        a.iter_mut()
            .enumerate()
            .filter(|(j, v)| (**v >> i) & 1 != 0 && *j != r)
            .for_each(|(_, v)| *v ^= now);
        r += 1;
    }

    let mut res = s;
    let mut k = 0;
    for i in (0..61).rev() {
        let nk = k | (1usize << i);
        let mut b = nk;
        for a in a.iter().take(r) {
            let top = 63 - a.leading_zeros();
            if (b >> top) & 1 != 0 {
                b ^= a;
            }
        }
        if b & nk == 0 {
            k = nk;
        }
    }

    res += k * 2;
    println!("{}", res);
}
