use proconio::*;

fn main() {
    input! {n: i64, m: usize, p: [(i64, i64); m]}

    let mut res = n * n;
    for &(a, b) in &p {
        res -= 2 * n;
        res -= b.min(n - a) + n - b;
        res -= b + (n - b).min(n - a);
    }

    for (i, &(a, b)) in p.iter().enumerate() {
        for &(s, t) in p.iter().skip(i + 1) {
            if s == a || t == b {
                let (mut a, mut b, mut s, mut t) = (a, b, s, t);
                if b == t {
                    (a, b, s, t) = (b, a, t, s);
                }
                if t > b {
                    (a, b, s, t) = (s, t, a, b);
                }
                res += n;
                if a >= b - t {
                    res += 2;
                }
                if (b - t) % 2 == 0 && (b - t) / 2 <= a {
                    res += 1;
                }
                if a <= n + 1 - (b - t) {
                    res += 2;
                }
                if (b - t) % 2 == 0 && a <= n + 1 - (b - t) / 2 {
                    res += 1;
                }
            } else if (s - a).abs() == (t - b).abs() {
            } else {
                res += 12;
            }
        }
    }

    println!("{res}")
}
