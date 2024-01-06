use proconio::{input, marker::Chars};
use segtree::{Monoid, SegmentTree};

struct U32Or;

impl Monoid for U32Or {
    type M = u32;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l | r
    }
}

fn main() {
    input! {n: usize, mut s: Chars, q: usize}

    let mut st = SegmentTree::<U32Or>::new(n);

    for (i, &c) in s.iter().enumerate() {
        st.set(i, 1 << (c as u8 - b'a'));
    }

    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {i: usize, c: char}

            if s[i - 1] == c {
                continue;
            }

            let old_c = s[i - 1];
            let val = (1 << (old_c as u8 - b'a')) | (1 << (c as u8 - b'a'));
            st.update_by(i - 1, |&old| old ^ val);

            s[i - 1] = c;
        } else {
            input! {l: usize, r: usize}

            let k = st.fold(l - 1..r);
            println!("{}", k.count_ones());
        }
    }
}
