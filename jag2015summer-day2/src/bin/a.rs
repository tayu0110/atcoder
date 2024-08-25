use math::MathInt;
use proconio::*;

fn main() {
    input! {p: usize, q: usize}

    let g = p.gcd(q);
    let q = q / g;
    println!("{}", q.factorize().map(|a| a.0).product::<usize>())
}
