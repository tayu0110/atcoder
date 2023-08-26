use proconio::*;

fn main() {
    input! {n: i128, a: i128, b: i128, c: i128, d: i128, x: f64}
    let x = (x * 1000.0).round() as i128;

    let (mut l, mut r) = (-1, 1000_000_000_000_000_000);
    while r - l > 1 {
        let m = (r + l) / 2;
        let cnt = n + m;
        let num = a + m + 2 * b + 3 * c + 4 * d;
        if cnt * x >= num * 1000 {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r)
}
