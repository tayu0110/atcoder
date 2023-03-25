#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n]}

    let op = |t, l, r| {
        if t == 1 {
            (l as f64, r as f64)
        } else if t == 2 {
            (l as f64, r as f64 - 0.1)
        } else if t == 3 {
            (l as f64 + 0.1, r as f64)
        } else {
            (l as f64 + 0.1, r as f64 - 0.1)
        }
    };

    let mut res = 0;
    for i in 0..n {
        let (t, l, r) = p[i];
        let (l, r) = op(t, l, r);
        for j in i+1..n {
            let (t2, l2, r2) = p[j];
            let (l2, r2) = op(t2, l2, r2);

            if !(r < l2 || r2 < l) {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
