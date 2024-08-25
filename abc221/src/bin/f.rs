#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

type Mint = modint::Mint<modint::Mod998244353>;

fn bfs(start: usize, dist: &mut [usize], t: &[Vec<usize>]) -> usize {
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((start, 0));

    let mut max = 0;
    while let Some((now, nd)) = nt.pop_front() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;
        max = std::cmp::max(max, nd);

        for to in &t[now] {
            if dist[*to] == std::usize::MAX {
                nt.push_back((*to, nd+1));
            }
        }
    }

    max
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    if n == 2 {
        println!("1");
        std::process::exit(0);
    }

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let max = bfs(0, &mut dist, &t);
    let root = dist.iter().enumerate().filter(|(_, v)| **v == max).fold(0, |_, (i, _)| i);
    
    let mut nd = vec![std::usize::MAX; n];
    let max = bfs(root, &mut nd, &t);

    let root = nd.iter().enumerate().filter(|(_, v)| **v == max).fold(0, |_, (i, _)| i);
    let mut dist = vec![std::usize::MAX; n];
    bfs(root, &mut dist, &t);

    if max % 2 == 1 {
        let middle = (0..n)
                .filter(|v| std::cmp::min(dist[*v], nd[*v]) == max / 2 && std::cmp::max(dist[*v], nd[*v]) == max / 2 + 1)
                .collect::<Vec<_>>();
        let mut dist = vec![std::usize::MAX; n];
        let mut buf = vec![vec![]; 2];
        for i in 0..2 {
            for to in &t[middle[i]] {
                if middle.iter().any(|v| v == to) {
                    continue;
                }
                let mut nt = std::collections::VecDeque::new();
                nt.push_back((*to, 1));
                let mut k = 0i64;
                while let Some((now, nd)) = nt.pop_front() {
                    if dist[now] != std::usize::MAX {
                        continue;
                    }
                    dist[now] = nd;
                    if nd == max / 2 {
                        k += 1;
                        continue;
                    }
                    for to in &t[now] {
                        if !middle.iter().any(|v| v == to) && dist[*to] == std::usize::MAX {
                            nt.push_back((*to, nd+1));
                        }
                    }
                }
    
                buf[i].push(k);
            }
        }
        let mut res = Mint::zero();
        let sum = Mint::new(buf[0].iter().sum::<i64>());
        for v in &buf[1] {
            res += sum * Mint::raw(*v);
        }
        println!("{}", res);
        std::process::exit(0);
    }

    let middle = (0..n).find(|v| dist[*v] == max / 2 && nd[*v] == max / 2).unwrap();

    let mut dist = vec![std::usize::MAX; n];
    let mut buf = vec![];
    for to in &t[middle] {
        let mut nt = std::collections::VecDeque::new();
        nt.push_back((*to, 1));
        let mut k = 0i64;
        while let Some((now, nd)) = nt.pop_front() {
            if dist[now] != std::usize::MAX {
                continue;
            }
            dist[now] = nd;
            if nd == max / 2 {
                k += 1;
                continue;
            }
            for to in &t[now] {
                if dist[*to] == std::usize::MAX && *to != middle {
                    nt.push_back((*to, nd+1));
                }
            }
        }

        buf.push(k);
    }

    let res = 
            buf.iter().fold(Mint::one(), |s, v| s * Mint::raw(*v+1))
          - buf.iter().fold(Mint::zero(), |s, v| s + Mint::raw(*v)) 
          - Mint::one();
    println!("{}", res);
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
