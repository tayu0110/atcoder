use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}
    const MOD: i64 = 998244353;

    let op = |l: i64, r: i64| (l + r) % MOD;
    let e = || 0i64;
    let id = || (1i64, 0i64);
    let mapping = |f: (i64, i64), x: i64| (x*f.0 + f.1) % MOD;
    let composition = |f: (i64, i64), g: (i64, i64)| (f.0*g.0 % MOD, (f.1*g.0+g.1) % MOD);

    let mut map = std::collections::BTreeMap::new();
    for v in &a {
        map.insert(*v, 0);
    }
    let mut size = 0;
    for (_, v) in map.iter_mut() {
        *v = size;
        size += 1;
    }

    let mut st = segtree::LazySegtree::new(size, op, e, id, mapping, composition);
    let mut res = 0;
    for v in a {
        let idx = *map.get(&v).unwrap();
        res += st.prod(0, idx+1);
        res %= MOD;

        st.apply_range(0, size, (2, 0));
        st.apply(idx, (1, 1));
    }

    println!("{}", res);
}

mod segtree {
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
        composition: fn(F, F) -> F
    }
    #[allow(dead_code)]
    impl<S, F> LazySegtree<S, F>
    where
        S: Copy + Clone + Sized,
        F: Copy + Clone + Sized
    {
        pub fn new(size: usize, op: fn(S, S) -> S, e: fn() -> S, id: fn() -> F, mapping: fn(F, S) -> S, composition: fn(F, F) -> F) -> Self {
            LazySegtree::from_vec(&vec![e(); size], op, e, id, mapping, composition)
        }
        pub fn from_vec(v: &[S], op: fn(S, S) -> S, e: fn() -> S, id: fn() -> F, mapping: fn(F, S) -> S, composition: fn(F, F) -> F) -> Self {
            let n = v.len();
            let (log, size) = {
                let (mut size, mut log) = (1, 0);
                while size < n { size <<= 1; log += 1; }
                (log, size)
            };
            let mut tree = vec![e(); size * 2];
            let lazy = vec![id(); size * 2];
            for (i, w) in v.iter().enumerate() { tree[size + i] = *w; }
            for i in (1..size).rev() { tree[i] = op(tree[i*2], tree[i*2 + 1]); }
            Self { n, size, log, tree, lazy, op, e, id, mapping, composition }
        }
        // Get the result of applying the operation to the interval [l, r).
        pub fn prod(&mut self, l: usize, r: usize) -> S {
            assert!(l <= r && r <= self.n);
            if l == r { return self.e(); }
            let (mut l, mut r) = (l + self.size, r + self.size);
            for i in (1..self.log+1).rev() {
                if ((l >> i) << i) != l { self.push(l >> i); }
                if ((r >> i) << i) != r { self.push(r >> i); }
            }
            let (mut sml, mut smr) = (self.e(), self.e());
            while l < r {
                if (l & 1) != 0 { sml = self.op(sml, self.tree[l]); l += 1; }
                if (r & 1) != 0 { r -= 1; smr = self.op(self.tree[r], smr); }
                l >>= 1; r >>= 1;
            }
            self.op(sml, smr)
        }
        // Apply val to a point whose index is idx.
        pub fn apply(&mut self, idx: usize, val: F) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (1..self.log+1).rev() { self.push(idx >> i); }
            self.tree[idx] = self.mapping(val, self.tree[idx]);
            for i in 1..=self.log { self.update(idx >> i); }
        }
        // Apply val to the interval [l, r).
        pub fn apply_range(&mut self, l: usize, r: usize, val: F) {
            assert!(l <= r && r <= self.n);
            if l == r { return; }
            let (l, r) = (l + self.size, r + self.size);
            for i in (1..self.log+1).rev() {
                if ((l >> i) << i) != l { self.push(l >> i); }
                if ((r >> i) << i) != r { self.push((r-1) >> i); }
            }
            let (mut a, mut b) = (l, r);
            while a < b {
                if (a & 1) != 0 { self.all_apply(a, val); a += 1; }
                if (b & 1) != 0 { b -= 1; self.all_apply(b, val); }
                a >>= 1; b >>= 1;
            }
            for i in 1..=self.log {
                if ((l >> i) << i) != l { self.update(l >> i); }
                if ((r >> i) << i) != r { self.update((r-1) >> i); }
            }
        }
        fn update(&mut self, idx: usize) { self.tree[idx] = self.op(self.tree[idx*2], self.tree[idx*2+1]); }
        fn all_apply(&mut self, idx: usize, val: F) {
            let mapping = self.mapping;
            self.tree[idx] = mapping(val, self.tree[idx]);
            if idx < self.size { self.lazy[idx] = self.composition(val, self.lazy[idx]); }
        }
        fn push(&mut self, idx: usize) {
            self.all_apply(idx*2, self.lazy[idx]);
            self.all_apply(idx*2 + 1, self.lazy[idx]);
            self.lazy[idx] = self.id();
        }
        fn op(&self, l: S, r: S) -> S { let op = self.op; op(l, r) }
        fn e(&self) -> S { let e = self.e; e() }
        fn id(&self) -> F { let id = self.id; id() }
        fn mapping(&self, f: F, x: S) -> S { let mapping = self.mapping; mapping(f, x) }
        fn composition(&self, f: F, g: F) -> F { let composition = self.composition; composition(f, g) }
    }
}
