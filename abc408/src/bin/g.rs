use proconio::*;

fn solve(a: usize, b: usize, c: usize, d: usize) -> (usize, usize) {
    let n = a / b;
    let a = a - n * b;
    let c = c - n * d;
    if c > d {
        return (n + 1, 1);
    }
    let (p, q) = solve(d, c, b, a);
    (p * n + q, p)
}

#[fastout]
fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {a: usize, b: usize, c: usize, d: usize}
        let (_, q) = solve(a, b, c, d);
        println!("{q}")
    }
}
