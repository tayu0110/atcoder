use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| (n - (a + i) % n) % n)
        .collect::<Vec<_>>();

    let g = gcd(n, m);

    if a.iter().sum::<usize>() % g != 0 {
        println!("-1");
    }
}
