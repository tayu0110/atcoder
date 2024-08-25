use iolib::*;
use math::divisors_enumeration;

fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    scan!(n: usize, k: usize, a: [u32; k]);

    let g = a
        .windows(2)
        .map(|v| v[0].abs_diff(v[1]))
        .reduce(gcd)
        .unwrap();

    let divs = divisors_enumeration(g as u64);

    let &min = a.iter().min().unwrap();
    let &max = a.iter().max().unwrap();

    let mut res = divs
        .into_iter()
        .filter(|&d| (max - min) / (d as u32) < n as u32)
        .collect::<Vec<_>>();
    res.sort_unstable();
    putln!(res.len());
    putitln!(res.into_iter(), sep = ' ');
}
