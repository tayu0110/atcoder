use proconio::input;

fn main() {
    input! {n: usize, s: usize, t: usize, a: usize, b: usize}

    let x = |n, a, b, k| (k - 1) as f64 / 2.0 * a as f64 + b as f64 / k as f64 * n as f64 ;

    let (mut l, mut r) = (1, t+1);
    while r - l > 1 {
        let k = (r + l) / 2;
        if x(n, a, b, k-1) > x(n, a, b, k) {
            l = k;
        } else {
            r = k;
        }
    }

    let k = l;
    let x = x(n, a, b, k);
    if s >= t - k + 1 && s <= t {
        println!("{}", (t-s) * a);
    } else {
        println!("{}", x);
    }
}