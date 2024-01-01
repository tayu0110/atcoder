use montgomery_modint::{Mod998244353, MontgomeryModint};
use polynomial::Polynomial;
use proconio::*;

fn main() {
    input! {q: usize, k: usize, query: [(char, usize); q]}

    let mut poly = Polynomial::<Mod998244353>::from(vec![1]);
    for (ty, x) in query {
        if x > k {
            if poly.deg() <= k {
                println!("0")
            } else {
                assert!(poly[k].val() != 998244353);
                println!("{}", poly[k].val());
            }
            continue;
        }
        let mut r = Polynomial::from(vec![0; x + 1]);
        r[0] = MontgomeryModint::one();
        r[x] = MontgomeryModint::one();
        if ty == '-' {
            r = r.inv(k + 1);
            r.shrink();
            // eprintln!("x: {x}, r_inv: {r:?}");
        }
        poly = poly * r;

        poly.shrink();

        if poly.deg() <= k {
            println!("0")
        } else {
            poly = poly.prefix(k + 1);
            assert!(poly[k].val() != 998244353);
            println!("{}", poly[k].val());
        }
    }

    // eprintln!("poly: {poly:?}")
}
