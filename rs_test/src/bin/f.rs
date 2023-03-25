#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
	input! {n: usize, a: [usize; n]}

    let mut f = vec![a[0]];
    let mut b = vec![a[n-1]];

    for i in 1..n {
        f.push(gcd(*f.last().unwrap(), a[i]));
    }

    for i in (0..n-1).rev() {
        b.push(gcd(*b.last().unwrap(), a[i]));
    }
    b.reverse();

    let mut res = 0;
    for i in 0..n {
        let r = if i == 0 {
            b[1]
        } else if i == n-1 {
            f[n-2]
        } else {
            gcd(f[i-1], b[i+1])
        };

        res = std::cmp::max(res, r);
    }

    println!("{}", res);
}
