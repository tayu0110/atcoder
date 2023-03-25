#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: usize, w: usize, p: [(usize, usize); h]}
    let p = p.into_iter().map(|(a, b)| (a-1, b-1)).collect::<Vec<(_, _)>>();

    let mut map = (0..w).map(|i| (i, i)).collect::<std::collections::BTreeMap<_, _>>();
    let mut st = segtree::SegmentTree::new(w, std::usize::MAX, |l, r| std::cmp::min(l, r));
    for i in 0..w {
        st.update(i, 0);
    }

    for (i, &(a, b)) in p.iter().enumerate() {
        let mut c = None;
        while let Some((x, y)) = map.range(a..=b).next().map(|(&x, &y)| (x, y)) {
            c = std::cmp::max(c, Some(y));
            st.update(x, std::usize::MAX);
            map.remove(&x);
        }
        if let Some(c) = c {
            if b + 1 < w {
                let entry = map.entry(b+1).or_insert(c);
                *entry = std::cmp::max(*entry, c);
                st.update(b+1, b+1-map.get(&(b+1)).cloned().unwrap_or_default());
            }
        }
        let res = st.get(0, w).saturating_add(i+1);
        if res == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", res);
        }
    }
}

mod segtree {
    #[allow(dead_code)]
    pub struct SegmentTree<T>
    where T: Clone + Copy + Sized {
        sz: usize,
        def_val: T,
        update_func: fn(T, T) -> T,
        tree: Vec<T>
    }
    #[allow(dead_code)]
    impl<T> SegmentTree<T>
    where T: Clone + Copy + Sized {
        pub fn new(size: usize, def_val: T, update_func: fn(T, T) -> T) -> Self {
            let mut sz = 1;
            while sz < size { sz <<= 1; }
            let tree = vec![def_val; sz * 2];
            Self { sz, def_val, update_func, tree }
        }
        pub fn update(&mut self, mut index: usize, val: T) {
            index += self.sz;
            self.tree[index] = val;
            let update_func = &self.update_func;
            while index >> 1 > 0 { index >>= 1; self.tree[index] = update_func(self.tree[index*2], self.tree[index*2+1]); }
        }
        fn get_sub(&self, left: usize, right: usize, now: usize, a: usize, b: usize) -> T {
            if right <= a || b <= left { return self.def_val; }
            if left <= a && b <= right { return self.tree[now]; }
            let update_func = &self.update_func;
            let mut res = self.def_val;
            res = update_func(res, self.get_sub(left, right, now*2  , a, (a+b) / 2));
            res = update_func(res, self.get_sub(left, right, now*2+1, (a+b) / 2, b));
            res
        }
        pub fn get(&self, left: usize, right: usize) -> T { self.get_sub(left, right, 1, 0, self.sz) }
    }
}

