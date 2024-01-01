use proconio::*;

fn floor(t: i64, m: i64) -> i64 {
    if t >= 0 {
        t / m
    } else {
        t.div_euclid(m)
    }
}

fn ceil(t: i64, m: i64) -> i64 {
    if t >= 0 {
        (t + m - 1) / m
    } else {
        t / m
    }
}

fn main() {
    input! {a: i64, m: i64, mut l: i64, mut r: i64}

    l -= a;
    r -= a;

    println!("{}", floor(r, m) - ceil(l, m) + 1)
}
