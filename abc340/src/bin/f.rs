use proconio::*;

fn ext_gcd(x: i128, y: i128) -> Option<(i128, i128)> {
    // is the result of xb - ay = 2 existed?
    // xb - ay = 1
    // x1 - 0y = x
    // x0 - 1y = -y
    let (mut sb, mut sa) = (1, 0);
    let (mut tb, mut ta) = (0, 1);
    let (mut s, mut t) = (x, -y);
    while t != 0 {
        let d = s / t;
        s -= d * t;
        sb -= d * tb;
        sa -= d * ta;

        (s, sb, sa, t, tb, ta) = (t, tb, ta, s, sb, sa);
    }
    if s < 0 {
        s *= -1;
        sb *= -1;
        sa *= -1;
    }
    if s == 1 {
        Some((sb * 2, sa * 2))
    } else if s == 2 {
        Some((sb, sa))
    } else {
        None
    }
}

const MINAB: i128 = -1000_000_000_000_000_000;
const MAXAB: i128 = 1000_000_000_000_000_000;

fn main() {
    input! {x: i128, y: i128}

    let mut res = vec![];
    if let Some((b, a)) = ext_gcd(x, y) {
        if x * b >= a * y && (MINAB..=MAXAB).contains(&a) && (MINAB..=MAXAB).contains(&b) {
            res.push((a, b));
        }
    }
    if let Some((a, b)) = ext_gcd(y, x) {
        if a * y >= x * b && (MINAB..=MAXAB).contains(&a) && (MINAB..=MAXAB).contains(&b) {
            res.push((a, b));
        }
    }

    if res.is_empty() {
        println!("-1")
    } else {
        let (a, b) = res[0];
        assert_eq!((x * b - a * y).abs() % 2, 0);
        assert_eq!(1, (x * b - a * y).abs() / 2);
        println!("{} {}", res[0].0, res[0].1);
    }
}
