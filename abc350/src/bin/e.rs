use std::collections::HashMap;

use proconio::*;

fn rec(n: usize, a: usize, x: f64, y: f64, memo: &mut HashMap<usize, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }

    if let Some(&res) = memo.get(&n) {
        return res;
    }

    let mut res = y * 6.0;
    for i in 2..=6 {
        res += rec(n / i, a, x, y, memo);
    }
    res /= 5.0;

    res = res.min(x + rec(n / a, a, x, y, memo));
    memo.insert(n, res);
    res
}

fn main() {
    input! {n: usize, a: usize, x: f64, y: f64}
    println!("{}", rec(n, a, x, y, &mut HashMap::new()))
}
