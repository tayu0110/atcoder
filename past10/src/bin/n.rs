use convolution::arbitrary_convolution;
use proconio::*;

const N: usize = 200000;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut s = vec![0; N + 1];
    let mut t = vec![0; N];
    for a in a {
        s[a as usize] += 1;
        t[N - a as usize] += 1;
    }

    let c = arbitrary_convolution::<100_000_000_000>(s, t);
    println!("{}", c.into_iter().filter(|&c| c > 0).count());
}
