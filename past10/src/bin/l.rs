use proconio::*;

fn powm(mut exp: usize, m: usize) -> usize {
    let mut res = 1;
    let mut val = 10;
    while exp != 0 {
        if exp & 1 != 0 {
            res *= val;
            res %= m;
        }
        val *= val;
        val %= m;
        exp >>= 1;
    }
    res
}

fn solve(c: usize, d: usize, m: usize) -> usize {
    if d == 0 {
        0
    } else if d == 1 {
        c
    } else {
        let r = solve(c, d >> 1, m);
        let res = (r * powm(d >> 1, m) + r) % m;
        if d & 1 != 0 {
            (res * 10 + c) % m
        } else {
            res
        }
    }
}

fn main() {
    input! {k: usize, m: usize, e: [(usize, usize); k]}

    let mut res = 0;
    for (c, d) in e {
        res = res * powm(d, m) + solve(c, d, m);
        res %= m;
    }

    println!("{res}")
}
