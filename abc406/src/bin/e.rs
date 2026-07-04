use montgomery_modint::Mod998244353;
use proconio::*;
use rand::{thread_rng, Rng};
use static_modint::{combination, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {t: usize}
    input! {query: [(usize, u32); t]}
    // let mut rng = thread_rng();
    // let query = (0..t)
    //     .map(|_| (rng.gen_range(0usize..100), rng.gen_range(0..15)))
    //     .collect::<Vec<_>>();

    let com = combination::<Mod998244353>(100);
    for (n, k) in query {
        let mut base = 0;
        let mut nn = Modint::zero();
        let mut res = Modint::zero();
        for i in (0..61).rev() {
            if n & (1usize << i) != 0 {
                if base >= k {
                    continue;
                }

                if i > 0 {
                    let t = com(i - 1, k - base - 1);
                    for j in 0..i {
                        res += t * Modint::new(1u64 << j);
                    }
                    res += nn * com(i, k - base);
                }

                nn += Modint::new(1u64 << i);
                base += 1;
                if k == base {
                    res += nn;
                }
            }
        }

        // let expected = (1..=n)
        //     .filter_map(|i| (i.count_ones() == k).then_some(i))
        //     .sum::<usize>();
        // assert_eq!(
        //     res,
        //     Modint::new(expected as u64),
        //     "n: {n}, n: {n:0b} k: {k}"
        // );

        println!("{res}")
    }
}
