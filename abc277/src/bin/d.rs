#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}

    let res = a.iter().sum::<usize>();
    a.sort();
    for i in 0..n {
        a.push(a[i] + m);
    }

    let mut num = 1;
    let mut sum = a[0];
    let mut max = 0;
    for (i, v) in a.windows(2).enumerate() {
        if v[1] - v[0] <= 1 {
            sum += v[1] % m;
            num += 1;
            if num > n {
                sum -= a[i+1-n] % m;
                num = n;
            }
        } else {
            sum = v[1] % m;
            num = 1;
        }
        max = std::cmp::max(max, sum);
    }

    println!("{}", res - max);
}
