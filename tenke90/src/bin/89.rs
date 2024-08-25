use proconio::input;

mod segtree {
    use std::fmt::Debug;

    #[allow(dead_code)]
    pub struct SegmentTree<T>
    where
        T: Clone + Copy + Sized,
    {
        sz: usize,
        def_val: T,
        update_func: fn(T, T) -> T,
        tree: Vec<T>,
    }
    #[allow(dead_code)]
    impl<T> SegmentTree<T>
    where
        T: Clone + Copy + Sized,
    {
        pub fn new(size: usize, def_val: T, update_func: fn(T, T) -> T) -> Self {
            let mut sz = 1;
            while sz < size {
                sz <<= 1;
            }
            let tree = vec![def_val; sz * 2];
            Self {
                sz,
                def_val,
                update_func,
                tree,
            }
        }
        pub fn update(&mut self, mut index: usize, val: T) {
            index += self.sz;
            self.tree[index] = val;
            let update_func = &self.update_func;
            while index >> 1 > 0 {
                index >>= 1;
                self.tree[index] = update_func(self.tree[index * 2], self.tree[index * 2 + 1]);
            }
        }
        fn get_sub(&self, left: usize, right: usize, now: usize, a: usize, b: usize) -> T {
            if right <= a || b <= left {
                return self.def_val;
            }
            if left <= a && b <= right {
                return self.tree[now];
            }
            let update_func = &self.update_func;
            let mut res = self.def_val;
            res = update_func(res, self.get_sub(left, right, now * 2, a, (a + b) / 2));
            res = update_func(res, self.get_sub(left, right, now * 2 + 1, (a + b) / 2, b));
            res
        }
        pub fn get(&self, left: usize, right: usize) -> T {
            self.get_sub(left, right, 1, 0, self.sz)
        }
    }
    impl<T> std::fmt::Debug for SegmentTree<T>
    where
        T: Copy + Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut buf = vec![];
            for i in 0..self.sz {
                buf.push(self.get(i, i + 1));
            }
            write!(f, "{:?}", buf)
        }
    }
    #[allow(dead_code)]
    pub struct LazySegtree<S, F> {
        n: usize,
        size: usize,
        log: usize,
        pub tree: Vec<S>,
        pub lazy: Vec<F>,
        op: fn(S, S) -> S,
        e: fn() -> S,
        id: fn() -> F,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F,
    }
    #[allow(dead_code)]
    impl<S, F> LazySegtree<S, F>
    where
        S: Copy + Clone + Sized,
        F: Copy + Clone + Sized,
    {
        pub fn new(
            size: usize,
            op: fn(S, S) -> S,
            e: fn() -> S,
            id: fn() -> F,
            mapping: fn(F, S) -> S,
            composition: fn(F, F) -> F,
        ) -> Self {
            LazySegtree::from_vec(&vec![e(); size], op, e, id, mapping, composition)
        }
        pub fn from_vec(
            v: &[S],
            op: fn(S, S) -> S,
            e: fn() -> S,
            id: fn() -> F,
            mapping: fn(F, S) -> S,
            composition: fn(F, F) -> F,
        ) -> Self {
            let n = v.len();
            let (log, size) = {
                let (mut size, mut log) = (1, 0);
                while size < n {
                    size <<= 1;
                    log += 1;
                }
                (log, size)
            };
            let mut tree = vec![e(); size * 2];
            let lazy = vec![id(); size * 2];
            for (i, w) in v.iter().enumerate() {
                tree[size + i] = *w;
            }
            for i in (0..size).skip(1).rev() {
                tree[i] = op(tree[i * 2], tree[i * 2 + 1]);
            }
            Self {
                n,
                size,
                log,
                tree,
                lazy,
                op,
                e,
                id,
                mapping,
                composition,
            }
        }
        pub fn set(&mut self, idx: usize, val: S) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log + 1).skip(1).rev() {
                self.push(idx >> i);
            }
            self.tree[idx] = val;
            for i in 1..=self.log {
                self.update(idx >> i);
            }
        }
        // Get the value of a single point whose index is idx.
        pub fn get(&mut self, idx: usize) -> S {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log + 1).skip(1).rev() {
                self.push(idx >> i);
            }
            self.tree[idx]
        }
        // Get the result of applying the operation to the interval [l, r).
        pub fn prod(&mut self, l: usize, r: usize) -> S {
            assert!(l <= r && r <= self.n);
            if l == r {
                return self.e();
            }
            let (mut l, mut r) = (l + self.size, r + self.size);
            for i in (0..self.log + 1).skip(1).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }
            let (mut sml, mut smr) = (self.e(), self.e());
            while l < r {
                if (l & 1) != 0 {
                    sml = self.op(sml, self.tree[l]);
                    l += 1;
                }
                if (r & 1) != 0 {
                    r -= 1;
                    smr = self.op(self.tree[r], smr);
                }
                l >>= 1;
                r >>= 1;
            }
            self.op(sml, smr)
        }
        pub fn all_prod(&self) -> S {
            self.tree[1]
        }
        // Apply val to a point whose index is idx.
        pub fn apply(&mut self, idx: usize, val: F) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log + 1).skip(1).rev() {
                self.push(idx >> i);
            }
            self.tree[idx] = self.mapping(val, self.tree[idx]);
            for i in 1..=self.log {
                self.update(idx >> i);
            }
        }
        // Apply val to the interval [l, r).
        pub fn apply_range(&mut self, l: usize, r: usize, val: F) {
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }
            let (l, r) = (l + self.size, r + self.size);
            for i in (0..self.log + 1).skip(1).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }
            let (mut a, mut b) = (l, r);
            while a < b {
                if (a & 1) != 0 {
                    self.all_apply(a, val);
                    a += 1;
                }
                if (b & 1) != 0 {
                    b -= 1;
                    self.all_apply(b, val);
                }
                a >>= 1;
                b >>= 1;
            }
            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }
        fn update(&mut self, idx: usize) {
            self.tree[idx] = self.op(self.tree[idx * 2], self.tree[idx * 2 + 1]);
        }
        fn all_apply(&mut self, idx: usize, val: F) {
            let mapping = self.mapping;
            self.tree[idx] = mapping(val, self.tree[idx]);
            if idx < self.size {
                self.lazy[idx] = self.composition(val, self.lazy[idx]);
            }
        }
        fn push(&mut self, idx: usize) {
            self.all_apply(idx * 2, self.lazy[idx]);
            self.all_apply(idx * 2 + 1, self.lazy[idx]);
            self.lazy[idx] = self.id();
        }
        fn op(&self, l: S, r: S) -> S {
            let op = self.op;
            op(l, r)
        }
        fn e(&self) -> S {
            let e = self.e;
            e()
        }
        fn id(&self) -> F {
            let id = self.id;
            id()
        }
        fn mapping(&self, f: F, x: S) -> S {
            let mapping = self.mapping;
            mapping(f, x)
        }
        fn composition(&self, f: F, g: F) -> F {
            let composition = self.composition;
            composition(f, g)
        }
    }
    impl<S, F> std::fmt::Debug for LazySegtree<S, F>
    where
        S: Copy + Clone + std::fmt::Debug,
        F: Copy + Clone + std::fmt::Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (mut tree, mut lazy, mut l, mut r) = (vec![], vec![], 1, 2);
            while r < self.tree.len() {
                let (mut nd, mut nlz) = (vec![], vec![]);
                for i in l..r {
                    nd.push(self.tree[i]);
                    nlz.push(self.lazy[i]);
                }
                tree.push(nd);
                lazy.push(nlz);
                l <<= 1;
                r <<= 1;
            }
            write!(f, "tree: {:?}\nlz: {:?}", tree, lazy)
        }
    }
    const MOD: i64 = 1000000007;
    #[allow(dead_code)]
    pub fn range_add_range_sum_query(size: usize) -> LazySegtree<(i64, i64), i64> {
        LazySegtree::from_vec(
            &vec![(0i64, 1i64); size],
            |l, r| ((l.0 + r.0) % MOD, l.1 + r.1),
            || (0, 0),
            || 0i64,
            |f, x| ((x.0 + f * x.1) % MOD, x.1),
            |f, g| (f + g) % MOD,
        )
    }
    #[allow(dead_code)]
    pub fn range_add_range_maximum_query(size: usize) -> LazySegtree<i64, i64> {
        LazySegtree::from_vec(
            &vec![0i64; size],
            std::cmp::max,
            || -9223372036854775808i64,
            || 0i64,
            |f, x| f + x,
            |f, g| f + g,
        )
    }
    #[allow(dead_code)]
    pub fn range_add_range_minimum_query(size: usize) -> LazySegtree<i64, i64> {
        LazySegtree::from_vec(
            &vec![0i64; size],
            std::cmp::min,
            || 0x7FFFFFFFFFFFFFFFi64,
            || 0i64,
            |f, x| f + x,
            |f, g| f + g,
        )
    }
    // Range Add Range Maximum Query: F: i64, S: i64, from_vec(&vec![0i64; size], |l, r| std::cmp::max(l, r), || -111222333444555666i64, || 0i64, |f, x| f + x, |f, g| f + g);
    // Range Add Range Minimum Query: F: i64, S: i64, from_vec(&vec![0i64; size], |l, r| std::cmp::min(l, r), || 111222333444555666i64, || 0i64, |f, x| f + x, |f, g| f + g);
    // Range Add Range Sum Query: F: i64, S: (i64, i64), from_vec(&vec![(0i64, 1i64); size], |l, r| (l.0+r.0, l.1+r.1), || (0, 0), || 0i64, |f, x| (x.0+f*x.1, x.1), |f, g| f + g);
}
#[allow(unused_imports)]
use segtree::LazySegtree;
#[allow(unused_imports)]
use segtree::SegmentTree;

fn main() {
    input! {n: usize, k: i64, a: [usize; n]};

    let mut map = std::collections::BTreeMap::new();
    let mut b = a.clone();
    b.sort();
    let mut cnt = 0;
    for v in b {
        if let std::collections::btree_map::Entry::Vacant(e) = map.entry(v) {
            e.insert(cnt);
            cnt += 1;
        }
    }

    let mut l = 0;
    let mut r = 0;
    let mut st = SegmentTree::new(cnt, 0i64, |l, r| l + r);
    let mut sum = 0i64;
    let mut to = vec![0; n + 1];
    while l < n {
        r = std::cmp::max(l, r);
        while r < n {
            let cr = *map.get(&a[r]).unwrap();
            let ss = st.get(cr + 1, cnt);
            if sum + ss > k {
                break;
            }
            sum += ss;
            let now = st.get(cr, cr + 1);
            st.update(cr, now + 1);
            r += 1;
        }
        to[l + 1] = r + 1;
        let cl = *map.get(&a[l]).unwrap();
        sum -= st.get(0, cl);
        let now = st.get(cl, cl + 1);
        st.update(cl, now - 1);
        l += 1;
    }

    let mut dp = segtree::range_add_range_sum_query(n + 1);
    dp.apply(0, 1);
    for i in 1..=n {
        let (tmp, _) = dp.get(i - 1);
        dp.apply_range(i, to[i], tmp);
    }

    println!("{}", dp.get(n).0);
}
