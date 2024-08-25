use modint::{Mod998244353, MontgomeryModint};
use proconio::input;

type Modint = MontgomeryModint<Mod998244353>;

fn main() {
    input! {t: usize}
    const MAX: usize = 2_000_010;

    let mut c = vec![Modint::zero(); MAX];
    c[1] = Modint::one();
    for i in 2..MAX {
        c[i] = c[i - 1] * Modint::new(i as u32);
    }

    for _ in 0..t {
        input! {n: Modint}

        if n == 2.into() {
            println!("{}", 1);
            continue;
        }

        let nn = n.val() as usize;

        println!(
            "{}",
            (n - Modint::one()) * n * n * (n * n - Modint::new(3)) * c[2 * nn - 4]
                / (c[nn] * c[nn])
        );
    }
}
