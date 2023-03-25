use proconio::input;

#[allow(dead_code)]
pub mod modint {
    use std::marker;
    use std::ops;
    pub trait Modulo {
        fn modulo() -> i64;
        fn primitive_root() -> i64;
    }
    #[derive(Clone, marker::Copy)]
    pub enum Mod998244353 {}
    impl Modulo for Mod998244353 {
        fn modulo() -> i64 { 998_244_353i64 }
        fn primitive_root() -> i64 { 3i64 }
    }
    #[derive(Clone, marker::Copy)]
    pub enum Mod1000000007 {}
    impl Modulo for Mod1000000007 {
        fn modulo() -> i64 { 1_000_000_007i64 }
        fn primitive_root() -> i64 { unimplemented!(); }
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Mint<M>
    where M: Modulo {
        val: i64,
        _p: marker::PhantomData<M>
    }
    impl<M> Mint<M>
    where M: Modulo + marker::Copy {
        pub fn new(val: i64) -> Self { Mint { val: val % M::modulo(), _p: marker::PhantomData } }
        pub fn raw(val: i64) -> Self { assert!(val < M::modulo()); Mint { val, _p: marker::PhantomData } }
        pub fn zero() -> Self { Mint { val: 0i64, _p: marker::PhantomData } }
        pub fn one() -> Self { Mint { val: 1i64, _p: marker::PhantomData } }
        pub fn modulo() -> i64 { M::modulo() }
        pub fn val(&self) -> i64 { self.val }
        pub fn pow(&self, mut exp: i64) -> Self {
            let (mut val, mut res) = (self.val, 1);
            while exp > 0 {
                if exp % 2 == 1 { res = (res * val) % M::modulo(); }
                val = (val * val) % M::modulo();
                exp >>= 1;
            }
            Self { val: res, _p: marker::PhantomData }
        }
        pub fn inv(&self) -> Self { self.pow(M::modulo() - 2) }
        pub fn nth_root(n: i64) -> Self {
            assert!(n.abs() == 1 << n.abs().trailing_zeros());
            assert!(M::modulo() - 1 + (M::modulo() - 1) / n >= 0);
            Mint::raw(M::primitive_root()).pow(M::modulo() - 1 + (M::modulo() - 1) / n)
        }
        pub fn add_raw(&self, rhs: i64) -> Self { *self + Mint::new(rhs) }
        pub fn sub_raw(&self, rhs: i64) -> Self { *self - Mint::new(rhs) }
        pub fn mul_raw(&self, rhs: i64) -> Self { *self * Mint::new(rhs) }
        pub fn div_raw(&self, rhs: i64) -> Self { *self / Mint::new(rhs) }
    }
    impl<M> Default for Mint<M> where M: Modulo + marker::Copy { fn default() -> Self { Mint::zero() } }
    impl<M> std::fmt::Debug for Mint<M> where M: Modulo + marker::Copy { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
    impl<M> std::fmt::Display for Mint<M> where M: Modulo + marker::Copy { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
    impl<M> ops::Add for Mint<M> where M: Modulo + marker::Copy { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self::new(self.val + rhs.val) } }
    impl<M> ops::AddAssign for Mint<M> where M: Modulo + marker::Copy { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
    impl<M> ops::Sub for Mint<M> where M: Modulo + marker::Copy { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self::new(self.val - rhs.val + M::modulo()) } }
    impl<M> ops::SubAssign for Mint<M> where M: Modulo + marker::Copy { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
    impl<M> ops::Mul for Mint<M> where M: Modulo + marker::Copy { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self::new(self.val * rhs.val) } }
    impl<M> ops::MulAssign for Mint<M> where M: Modulo + marker::Copy { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
    impl<M> ops::Div for Mint<M> where M: Modulo + marker::Copy { type Output = Self; fn div(self, rhs: Self) -> Self::Output { assert!(rhs.val != 0); self * rhs.inv() } }
    impl<M> ops::DivAssign for Mint<M> where M: Modulo + marker::Copy { fn div_assign(&mut self, rhs: Self) { assert!(rhs.val != 0); *self *= rhs.inv() } }
    pub struct Combination<M>
    where M: Modulo {
        fact: Vec<Mint<M>>,
        ifact: Vec<Mint<M>>
    }
    impl<M> Combination<M>
    where M: Modulo + marker::Copy {
        pub fn new(size: usize) -> Self {
            let mut fact = vec![Mint::one(); size+1];
            let mut ifact = vec![Mint::one(); size+1];
            let mut buf = vec![Mint::one(); size+1];
            fact.iter_mut().enumerate().skip(1).for_each(|(i, v)| { *v = buf[i-1] * Mint::raw(i as i64); buf[i] = *v; });
            ifact[size] = fact[size].inv();
            buf[size] = ifact[size];
            ifact.iter_mut().enumerate().skip(1).rev().skip(1).for_each(|(i, v)| { *v = buf[i+1] * Mint::raw(i as i64 + 1); buf[i] = *v; });
            Self { fact, ifact }
        }
        pub fn get(&self, n: usize, k: usize) -> Mint<M> {
            if n < k {
                Mint::zero()
            } else {
                self.fact[n] * self.ifact[k] * self.ifact[n-k]
            }
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
        pub fn from_vec(v: &Vec<S>, op: fn(S, S) -> S, e: fn() -> S, id: fn() -> F, mapping: fn(F, S) -> S, composition: fn(F, F) -> F) -> Self {
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
        pub fn set(&mut self, idx: usize, val: S) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (1..self.log+1).rev() { self.push(idx >> i); }
            self.tree[idx] = val;
            for i in 1..=self.log { self.update(idx >> i); }
        }
        // Get the value of a single point whose index is idx.
        pub fn get(&mut self, idx: usize) -> S {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (1..self.log+1).rev() { self.push(idx >> i); }
            self.tree[idx]
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
        pub fn all_prod(&self) -> S { self.tree[1] }
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
    impl<S, F> std::fmt::Debug for LazySegtree<S, F>
    where
        S: Copy + Clone + std::fmt::Debug,
        F: Copy + Clone + std::fmt::Debug
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (mut tree, mut lazy, mut l, mut r) = (vec![], vec![], 1, 2);
            while r < self.tree.len() {
                let (mut nd, mut nlz) = (vec![], vec![]);
                for i in l..r { nd.push(self.tree[i]); nlz.push(self.lazy[i]); }
                tree.push(nd); lazy.push(nlz);
                l <<= 1; r <<= 1;
            }
            write!(f, "tree: {:?}\nlz: {:?}", tree, lazy)
        }
    }
    #[allow(dead_code)]
    pub fn range_add_range_sum_query(size: usize) -> LazySegtree<(i64, i64), i64> {
        LazySegtree::from_vec(
            &vec![(0i64, 1i64); size],
            |l, r| (l.0+r.0, l.1+r.1),
            || (0, 0),
            || 0i64,
            |f, x| (x.0+f*x.1, x.1),
            |f, g| f + g)
    }
    #[allow(dead_code)]
    pub fn range_add_range_maximum_query(size: usize) -> LazySegtree<i64, i64> {
        LazySegtree::from_vec(
            &vec![0i64; size],
            |l, r| std::cmp::max(l, r),
            || -9223372036854775808i64,
            || 0i64,
            |f, x| f + x,
            |f, g| f + g)
    }
    #[allow(dead_code)]
    pub fn range_add_range_minimum_query(size: usize) -> LazySegtree<i64, i64> {
        LazySegtree::from_vec(
            &vec![0i64; size],
            |l, r| std::cmp::min(l, r),
            || 0x7FFFFFFFFFFFFFFFi64,
            || 0i64,
            |f, x| f + x,
            |f, g| f + g)
    }
    // Range Add Range Maximum Query: F: i64, S: i64, from_vec(&vec![0i64; size], |l, r| std::cmp::max(l, r), || -111222333444555666i64, || 0i64, |f, x| f + x, |f, g| f + g);
    // Range Add Range Minimum Query: F: i64, S: i64, from_vec(&vec![0i64; size], |l, r| std::cmp::min(l, r), || 111222333444555666i64, || 0i64, |f, x| f + x, |f, g| f + g);
    // Range Add Range Sum Query: F: i64, S: (i64, i64), from_vec(&vec![(0i64, 1i64); size], |l, r| (l.0+r.0, l.1+r.1), || (0, 0), || 0i64, |f, x| (x.0+f*x.1, x.1), |f, g| f + g);
}

type Mint = modint::Mint<modint::Mod998244353>;

fn main() {
    input! {n: usize, q: usize, a: [i64; n]};
    let a = a.into_iter().map(|v| (Mint::raw(v), Mint::one())).collect::<Vec<_>>();

    let op = |l: (Mint, Mint), r: (Mint, Mint)| (l.0 + r.0, l.1 + r.1);
    let e = || (Mint::zero(), Mint::zero());
    let id = || (Mint::one(), Mint::zero());
    let mapping = |f: (Mint, Mint), x: (Mint, Mint)| (f.0 * x.0 + f.1 * x.1, x.1);
    let composition = |f: (Mint, Mint), g: (Mint, Mint)| (f.0 * g.0, f.0 * g.1 + f.1);
    let mut st = segtree::LazySegtree::from_vec(&a, op, e, id, mapping, composition);

    let mut res = vec![];

    for _ in 0..q {
        input! {t: usize};
        if t == 0 {
            input! {l: usize, r: usize, b: i64, c: i64};
            st.apply_range(l, r, (Mint::raw(b), Mint::raw(c)));
        } else {
            input! {l: usize, r: usize};
            res.push(st.prod(l, r));
        }
    }

    for (v, _) in res {
        println!("{}", v);
    }
}