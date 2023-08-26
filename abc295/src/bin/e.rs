use proconio::input;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n]}

    let com = combination(n as u32 + 10);

    let mut res = Modint::zero();
    for i in 1..=m {
        let mut rem = n + 1 - k;
        let mut zero = 0;
        for &j in &a {
            if j >= i {
                rem = rem.wrapping_sub(1);
            }
            if j == 0 {
                zero += 1;
            }
        }

        if rem > zero {
            res += if rem > n {
                Modint::one()
            } else {
                Modint::zero()
            };
            continue;
        }

        let p = Modint::raw(m as u32 + 1 - i as u32) / Modint::raw(m as u32);
        for j in rem..=zero {
            res += com(zero, j) * p.pow(j as u32) * (Modint::one() - p).pow(zero as u32 - j as u32);
        }
    }

    println!("{}", res)
}
