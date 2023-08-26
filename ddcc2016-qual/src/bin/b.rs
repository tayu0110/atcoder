use proconio::*;

fn main() {
    input! {r: f64, n: i64, m: i64}

    let mut res = 0.0;
    let t = (2.0 * r) / n as f64;
    for i in 1..n + m {
        let j = i - m;
        let k = if i >= n {
            let p = t * j as f64;
            let h = (p - r).abs();
            (r * r - h * h).sqrt()
        } else if j < 0 {
            let p = t * i as f64;
            let h = (p - r).abs();
            (r * r - h * h).sqrt()
        } else {
            let s = (t * i as f64 - r).abs();
            let t = (t * j as f64 - r).abs();
            let s = r * r - s * s;
            let t = r * r - t * t;
            if s > t {
                s.sqrt()
            } else {
                t.sqrt()
            }
        };
        res += k * 2.0;
    }

    println!("{}", res)
}
