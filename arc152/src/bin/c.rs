use proconio::*;

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    input! {n: usize, a: [i64; n]}

    let (min, max) = (a[0], *a.last().unwrap());
    let d = max - min;

    let g = a.iter().fold(d, |s, v| gcd(s, 2 * (v - min)));
    println!("{}", min % g + d)
}
