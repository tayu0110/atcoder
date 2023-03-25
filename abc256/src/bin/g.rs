use modint::{Mod998244353, MontgomeryModint};
use proconio::input;
use segtree::SegmentTree;

type Modint = MontgomeryModint<Mod998244353>;

fn main() {
    input! {n: usize, q: usize, a: [Modint; n]}

    let mut i2a = SegmentTree::from_vec(
        &a.iter()
            .enumerate()
            .map(|(i, &a)| Modint::from(((i + 1) * (i + 1)) as u64) * a)
            .collect(),
        Modint::zero(),
        |&l, &r| l + r,
    );
    let mut ia = SegmentTree::from_vec(
        &a.iter()
            .enumerate()
            .map(|(i, &a)| Modint::from(i as u64 + 1) * a)
            .collect(),
        Modint::zero(),
        |&l, &r| l + r,
    );
    let mut sa = SegmentTree::from_vec(&a, Modint::zero(), |&l, &r| l + r);

    let div2 = Modint::raw(2).inv();
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, v: Modint}
            let nx = Modint::raw(x as u32);
            i2a.set(x - 1, nx * nx * v);
            ia.set(x - 1, nx * v);
            sa.set(x - 1, v);
        } else {
            input! {x: usize}

            let i2a = i2a.foldl(0, x);
            let ia = ia.foldl(0, x);
            let sa = sa.foldl(0, x);
            let x = Modint::raw(x as u32);
            println!(
                "{}",
                ((i2a + (x + Modint::one()) * (x + Modint::raw(2)) * sa
                    - (Modint::raw(2) * x + Modint::raw(3)) * ia)
                    * div2)
            )
        }
    }
}
