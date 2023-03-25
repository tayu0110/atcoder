#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, a: [usize; n]};

    let mut res = 0;
    let (mut l, mut r) = (0, 0);
    let mut sum = 0;
    while l < n {
        while r < n && sum + a[r] < k {
            sum += a[r];
            r += 1;
        }

        res += n - r;
        if l < r {
            sum -= a[l];
        } 
        l += 1;
        if r < l {
            r = l;
        }
    }

    println!("{}", res);
}
