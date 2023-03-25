#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n:usize, m: usize, a: [i64; n]};

    let mut b = a.iter().take(m).enumerate().fold(0, |s, (i, v)| s + *v * (i+1) as i64);
    let mut sum = a.iter().take(m).sum::<i64>();
    let mut res = b;
    // eprintln!("b: {}, sum: {}, res: {}", b, sum, res);

    for i in 0..n {
        if i + m >= n {
            break;
        }
        b -= sum;
        b += m as i64 * a[i+m];
        sum -= a[i];
        sum += a[i+m];
        res = std::cmp::max(res, b);

        // eprintln!("b: {}, sum: {}, res: {}", b, sum, res);
    }

    println!("{}", res);
}
