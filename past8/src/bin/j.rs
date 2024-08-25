use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct U8Xor;

impl MapMonoid for U8Xor {
    type E = u8;
    type Act = u8;
    fn e() -> Self::E {
        0
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        l ^ r
    }
    fn id() -> Self::Act {
        0
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l ^ r
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        act ^ elem
    }
}

fn main() {
    input! {n: usize, q: usize}

    let mut st = LazySegtree::<U8Xor>::new(2 * n);
    for _ in 0..q {
        input! {t: u8, k: usize}

        if t == 1 {
            if st.get(k - 1) == 0 {
                println!("{k}")
            } else {
                println!("{}", 2 * n - k + 1)
            }
        } else {
            st.apply_range(n - k, n + k, 1);
        }
    }
}
