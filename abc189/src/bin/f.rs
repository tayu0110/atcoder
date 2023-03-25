#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; k]}

    {
        let mut c = 0;
        for i in 0..k {
            let mut cnt = 1;
            for j in i..k-1 {
                if a[j+1] - a[j] != 1 {
                    break;
                }
                cnt += 1;
            }
            c = std::cmp::max(c, cnt);
        }

        if c >= m {
            println!("-1");
            std::process::exit(0);
        }
    }

    let a = a.into_iter().collect::<std::collections::HashSet<_>>();

    let mut t = vec![(0.0, 0.0); n + m];
    let mut sum = 0.0;
    let mut undef = 0.0;
    for i in (0..n).rev() {
        if a.contains(&i) {
            t[i] = (0.0, 1.0);
            let (s, u) = t[i+m];
            sum -= s;
            undef += 1.0 - u;
        } else {
            t[i] = (sum / m as f64 + 1.0, undef / m as f64);
            let (s, u) = t[i+m];
            sum += t[i].0 - s;
            undef += t[i].1 - u;
        }
    }

    println!("{}", t[0].0 / (1.0 - t[0].1));
}
