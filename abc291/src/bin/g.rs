use convolution::convolution;
use montgomery_modint::Mod998244353;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], mut b: [usize; n]}
    b.reverse();

    let mut t = vec![n * 31; n];
    for i in 0..5 {
        let na = a
            .iter()
            .map(|&a| (a & (1 << i) == 0) as u32)
            .collect::<Vec<_>>();
        let nb = b
            .iter()
            .map(|&b| (b & (1 << i) == 0) as u32)
            .collect::<Vec<_>>();

        let c = convolution::<Mod998244353>(na, nb);
        for (j, c) in c.into_iter().enumerate() {
            t[j % n] -= c as usize * (1 << i);
        }
    }

    println!("{}", t.into_iter().max().unwrap());
}
