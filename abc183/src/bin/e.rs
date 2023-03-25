#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

type Mint = modint::Mint<modint::Mod1000000007>;

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut dp = vec![vec![Mint::zero(); w]; h];
    dp[0][0] = Mint::one();
    let mut hdp = dp.clone();
    let mut vdp = dp.clone();
    let mut sdp = dp.clone();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            // let (mut ni, mut nj) = (i, j);
            // while nj > 0 && s[i][nj-1] == '.' {
            //     dp[i][j] = dp[i][j] + dp[i][nj-1];
            //     nj -= 1;
            // }
            if j > 0 && s[i][j-1] == '.' {
                dp[i][j] += hdp[i][j-1];
            }
            // while ni > 0 && s[ni-1][j] == '.' {
            //     dp[i][j] = dp[i][j] + dp[ni-1][j];
            //     ni -= 1;
            // }
            if i > 0 && s[i-1][j] == '.' {
                dp[i][j] += vdp[i-1][j];
            }
            // let (mut ni, mut nj) = (i, j);
            // while ni > 0 && nj > 0 && s[ni-1][nj-1] == '.' {
            //     dp[i][j] = dp[i][j] + dp[ni-1][nj-1];
            //     ni -= 1;
            //     nj -= 1;
            // }
            if i > 0 && j > 0 && s[i-1][j-1] == '.' {
                dp[i][j] += sdp[i-1][j-1];
            }
            if j > 0 && s[i][j-1] == '.' {
                hdp[i][j] = dp[i][j] + hdp[i][j-1];
            } else {
                hdp[i][j] = dp[i][j];
            }
            if i > 0 && s[i-1][j] == '.' {
                vdp[i][j] = dp[i][j] + vdp[i-1][j];
            } else {
                vdp[i][j] = dp[i][j];
            }
            if i > 0 && j > 0 && s[i-1][j-1] == '.' {
                sdp[i][j] = dp[i][j] + sdp[i-1][j-1];
            } else {
                sdp[i][j] = dp[i][j];
            }
        }
    }
    
    println!("{}", dp[h-1][w-1]);
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