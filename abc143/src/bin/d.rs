#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut l: [usize; n]}

    l.sort();

    let mut res = 0;
    for i in 0..n {
        for j in i+1..n {
            let k = l[i] + l[j];
            let (mut a, mut b) = (0, n);
            while b - a > 1 {
                let m = (a + b) / 2;
                if k <= l[m] {
                    b = m;
                } else {
                    a = m;
                }
            }

            if a <= j {
                continue;
            }

            res += a - j;
        }
    }

    println!("{}", res);
}
