use proconio::*;
use segtree::{LazySegtree, MapMonoid};

struct T;

impl MapMonoid for T {
    type E = Vec<usize>;
    type Act = usize;

    fn e() -> Self::E {
        vec![]
    }
    fn op(l: Self::E, r: Self::E) -> Self::E {
        if l.is_empty() {
            r
        } else if r.is_empty() {
            l
        } else {
            l.into_iter().zip(r).map(|(l, r)| l ^ r).collect()
        }
    }

    fn id() -> Self::Act {
        0
    }
    fn composite(l: Self::Act, r: Self::Act) -> Self::Act {
        l + r
    }

    fn map(mut act: Self::Act, mut elem: Self::E) -> Self::E {
        if !elem.is_empty() {
            act %= elem.len();
            elem.rotate_left(act);
        }
        elem
    }
}

fn main() {
    input! {n: usize, d: usize, a: [usize; n], q: usize}

    let mut st = LazySegtree::<T>::new(n);
    for (i, mut a) in a.into_iter().enumerate() {
        let mut t = vec![0; d];
        for i in 0..d {
            t[(d - i) % d] = a;
            a = a / 10 + a % 10 * 10usize.pow(d as u32 - 1);
        }
        st.set(i, t);
    }
    let mut base = 0;
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize}
            base = (base + x) % n;
        } else if ty == 2 {
            input! {l: usize, r: usize, y: usize}
            let l = (l - 1 + base) % n;
            let r = (r + base) % n;
            if l < r {
                st.apply_range(l, r, y);
            } else {
                st.apply_range(l, n, y);
                st.apply_range(0, r, y);
            }
        } else {
            input! {l: usize, r: usize}
            let l = (l - 1 + base) % n;
            let r = (r + base) % n;
            let res = if l < r {
                *st.prod(l, r).first().unwrap_or(&0)
            } else {
                st.prod(l, n).first().unwrap_or(&0) ^ st.prod(0, r).first().unwrap_or(&0)
            };
            println!("{res}")
        }
    }
}
