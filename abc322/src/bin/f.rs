use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct RangeFlipRangeLongestTerm;

impl MapMonoid for RangeFlipRangeLongestTerm {
    type E = (u32, u32, u32, u32, u32, u32, u32);
    type Act = bool;

    fn e() -> Self::E {
        (0, 0, 0, 0, 1, 1, 1)
    }
    fn apply(l: Self::E, r: Self::E) -> Self::E {
        let (rm, rl, rr, rt, rm0, rl0, rr0) = r;
        let (m, l, r, t, m0, l0, r0) = l;
        (
            rm.max(m).max(r + rl),
            if l == t { l + rl } else { l },
            if rr == rt { rr + r } else { rr },
            t + rt,
            rm0.max(m0).max(r0 + rl0),
            if l0 == t { l0 + rl0 } else { l0 },
            if rr0 == rt { rr0 + r0 } else { rr0 },
        )
    }
    fn id() -> Self::Act {
        false
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l ^ r
    }
    fn map(act: Self::Act, elem: Self::E) -> Self::E {
        if !act {
            return elem;
        }
        let (m, l, r, t, m0, l0, r0) = elem;
        (m0, l0, r0, t, m, l, r)
    }
}

fn main() {
    input! {_: usize, q: usize, s: marker::Chars}

    let mut st = LazySegtree::<RangeFlipRangeLongestTerm>::from_vec(
        &s.into_iter()
            .map(|c| {
                let c = c as u32 - b'0' as u32;
                (c, c, c, 1, 1 - c, 1 - c, 1 - c)
            })
            .collect(),
    );
    for _ in 0..q {
        input! {c: u8, l: usize, r: usize}

        if c == 1 {
            st.apply_range(l - 1, r, true);
        } else {
            let res = st.prod(l - 1, r);
            println!("{}", res.0)
        }
    }
}
