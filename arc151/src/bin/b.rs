#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

type Mint = modint::Mint<modint::Mod998244353>;

// #[fastout]
fn main() {
    input! {n: usize, m: i64, p: [usize; n]}
    let p = p.into_iter().map(|i| i - 1).collect_vec();
    
    let mut uf = unionfind::UnionFind::new(n);
    for i in 0..n {
        uf.merge(i, p[i]);
    }
    
    let mut set = std::collections::HashSet::new();
    let mut s = vec![];
    let mut size1 = Mint::one();
    for i in 0..n {
        let root = uf.root(i);
        if set.contains(&root) {
            continue;
        }
        
        let size = uf.size(root);
        if size == 1 {
            size1 *= Mint::new(m);
        } else {
            s.push(uf.size(root));
        }
        set.insert(root);
    }
    if s.is_empty() {
        println!("0");
        return;
    }
    
    let div2 = Mint::raw(2).inv();
    let groups = s.len();
    // 0: small, 1: equal
    let mut dp = vec![vec![Mint::zero(); groups+1]; 2];
    dp[1][0] = Mint::one();
    for (i, size) in s.into_iter().enumerate() {
        let com = Mint::new(m) * Mint::new(m-1) * div2;
        let mut p = Mint::one();
        let mut r = Mint::zero();
        for _ in 0..size-1 {
            r += com * p;
            p *= Mint::new(m);
        }

        dp[0][i+1] = dp[1][i] * r + dp[0][i] * p * Mint::new(m);
        dp[1][i+1] = dp[1][i] * Mint::new(m);
    }

    println!("{}", dp[0][groups] * size1);
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind { tree: vec![-1; size] }
        }
        pub fn root(&mut self, index: usize) -> usize {
            if self.tree[index] < 0 {
                index
            } else {
                self.tree[index] = self.root(self.tree[index] as usize) as i32;
                self.tree[index] as usize
            }
        }
        pub fn size(&mut self, index: usize) -> usize {
            let root = self.root(index);
            -self.tree[root] as usize
        }
        pub fn is_same(&mut self, left: usize, right: usize) -> bool {
            self.root(left) == self.root(right)
        }
        pub fn merge(&mut self, left: usize, right: usize) -> bool {
            let (mut rl, mut rr) = (self.root(left), self.root(right));
            if rl == rr {
                return false;
            }
            if self.tree[rl] > self.tree[rr] {
                std::mem::swap(&mut rl, &mut rr);
            }
            self.tree[rl] += self.tree[rr];
            self.tree[rr] = rl as i32;
            true
        }
    }
}

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
        pub fn new(val: i64) -> Self { Mint { val: (val % M::modulo() + M::modulo()) % M::modulo(), _p: marker::PhantomData } }
        pub fn raw(val: i64) -> Self { assert!(0 <= val && val < M::modulo()); Mint { val, _p: marker::PhantomData } }
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
