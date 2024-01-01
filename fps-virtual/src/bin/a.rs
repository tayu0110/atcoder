use montgomery_modint::Mod998244353;
use polynomial::Polynomial;
use proconio::*;

fn main() {
    input! {n: usize, p: [usize; n]}

    let mut res = Polynomial::<Mod998244353>::from(vec![1]);
    for p in p {
        let mut buf = vec![0; p + 1];
        buf[0] = 1;
        buf[p] = 1;
        res = res * Polynomial::from(buf);
    }

    let res: Vec<u32> = res.into();
    println!("{}", res.iter().filter(|&&f| f > 0).count())
}
