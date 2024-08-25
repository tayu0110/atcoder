#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}
    p.sort();

    let mut max_seg = segtree::SegmentTree::new(n, 0, std::cmp::max);
    let mut min_seg = segtree::SegmentTree::new(n, std::usize::MAX, std::cmp::min);

    let (px, py) = p.iter().cloned().unzip::<usize, usize, Vec<_>, Vec<_>>();
    for (i, &y) in py.iter().enumerate() {
        max_seg.update(i, y);
        min_seg.update(i, y);
    }

    let (mut l, mut r) = (0, 1 << 35);
    let mut res = 0;
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut inner_res = 0;

        {
            let (mut l, mut r) = (0, 0);
            let (mut pmin, mut pmax) = (std::usize::MAX, 0);
            while l < n {
                pmin = std::cmp::min(pmin, py[l]);
                pmax = std::cmp::max(pmax, py[l]);
                while r < n && px[r] - px[l] < m {
                    r += 1;
                }

                if r == n {
                    break;
                }

                if px[r] - px[l] >= m {
                    let nmin = min_seg.get(r, n);
                    let nmax = max_seg.get(r, n);

                    if nmax >= pmin {
                        inner_res = std::cmp::max(inner_res, nmax - pmin);
                        res = std::cmp::max(res, std::cmp::min(px[r] - px[l], nmax - pmin));
                    }

                    if pmax >= nmin {
                        inner_res = std::cmp::max(inner_res, pmax - nmin);
                        res = std::cmp::max(res, std::cmp::min(px[r] - px[l], pmax - nmin));
                    }
                }
                
                l += 1;
                if r < l {
                    r = l;
                }
            }
        }

        if inner_res < m {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", res);
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
