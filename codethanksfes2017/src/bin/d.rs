use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    input! {n: usize, m: usize}

    let g = gcd(n, m);
    println!("{}", m - g)
}
