use ds::{LazySegmentTree, MapMonoid};
use proconio::*;

struct T;
impl MapMonoid for T {
    type M = (bool, usize);
    type Act = (bool, usize, usize);

    fn e() -> Self::M {
        (false, 0)
    }

    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        (l.0 | r.0, l.1.max(r.1))
    }

    fn id() -> Self::Act {
        (false, 0, 0)
    }

    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {
        let (mut l, mut r) = (*l, *r);
        if l.2 < r.2 {
            (l, r) = (r, l);
        }

        if l.0 {
            return l;
        }
        (l.0 | r.0, l.1 + r.1, l.2)
    }

    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        if act.0 {
            if m.0 {
                (false, act.1)
            } else {
                (true, act.1)
            }
        } else {
            if m.0 {
                (true, 0)
            } else {
                (false, m.1 + act.1)
            }
        }
    }
}

fn main() {
    input! {n: usize, q: usize}

    let mut st = LazySegmentTree::<T>::new(n);
    for i in 1..=q {
        input! {ty: u8, l: usize, r: usize}

        if ty == 1 {
            input! {x: usize}
            st.apply(l - 1..r, (false, x, i));
        } else if ty == 2 {
            st.apply(l - 1..r, (true, 0, i));
        } else {
            println!("{}", st.fold(l - 1..r).1)
        }
    }
}
