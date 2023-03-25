use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut k: i64, mut a: [i64; n], mut f: [i64; n]};

    a.sort_by_key(|v| std::cmp::Reverse(*v));
    f.sort();

    let v = f.into_iter().zip(a.into_iter()).map(|(f, a)| (f, a)).collect_vec();

    let (mut l, mut r) = (-1, 0x3f3f3f3f3f3f3f3f);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut nk = 0;
        for (f, a) in &v {
            let target = m / *f;
            if target < *a {
                nk += *a - target;
            }
        }

        if nk > k {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", r);
}
