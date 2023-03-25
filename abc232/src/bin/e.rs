use proconio::input;

type Modint = modint::Mint<modint::Mod998244353>;

fn main() {
    input! {h: u32, w: u32, k: u32, x: u32, y: u32, c: u32, r: u32}

    let mut dp = vec![vec![Modint::zero(); 2]; 2];
    {
        let a = if x != c { 1 } else { 0 };
        let b = if y != r { 1 } else { 0 };
        dp[a][b] = Modint::one();
    }

    let row = vec![Modint::one(), Modint::new(h as u64 - 1)];
    let col = vec![Modint::one(), Modint::new(w as u64 - 1)];

    for _ in 0..k {
        let mut new = vec![vec![Modint::zero(); 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for a in 0..2 {
                    new[i][a] += dp[i][j] * (col[a] - if j == a { Modint::one() } else { Modint::zero() });
                    new[a][j] += dp[i][j] * (row[a] - if i == a { Modint::one() } else { Modint::zero() });
                }
            }
        }
        dp = new;
    }

    println!("{}", dp[0][0]);
}

pub mod modint {
    use std::convert::From;
    use std::marker::{self, PhantomData};
    use std::num::ParseIntError;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
    use std::str::FromStr;
    pub trait Modulo: Clone + marker::Copy + PartialEq + Eq {
        const MOD: u32;
        const MOD_INV: u32;
        const R: u32;
        const R2: u32;
        const PRIM_ROOT: u32;
        fn montgomery_constant_mask() -> u32 { unimplemented!() }
    }
    #[derive(Clone, marker::Copy, PartialEq, Eq)]
    pub enum Mod998244353 {}
    impl Modulo for Mod998244353 {
        const MOD: u32 = 998_244_353;
        // R = 2^32 mod 998244353
        const R: u32 = ((1u64 << 32) % Self::MOD as u64) as u32;
        // modulo * modulo_inv = -1 mod R
        const MOD_INV: u32 = {
            let inv = Self::MOD.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(Self::MOD)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            inv.wrapping_neg()
        };
        // R2 = 2^64 mod 998244353
        const R2: u32 = ((Self::MOD as u64).wrapping_neg() % Self::MOD as u64) as u32;
        const PRIM_ROOT: u32 = 3;
        #[inline]
        // R - 1 = 2^32 - 1
        fn montgomery_constant_mask() -> u32 { !0 }
    }
    #[derive(Clone, marker::Copy, PartialEq, Eq)]
    pub enum Mod1000000007 {}
    impl Modulo for Mod1000000007 {
        const MOD: u32 = 1_000_000_007;
        const MOD_INV: u32 = {
            let inv = Self::MOD.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(Self::MOD)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            let inv = inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(inv)));
            inv.wrapping_neg()
        };
        const PRIM_ROOT: u32 = 5;
        const R: u32 = ((1u64 << 32) % Self::MOD as u64) as u32;
        const R2: u32 = (Self::R as u64 * Self::R as u64 % Self::MOD as u64) as u32;
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Mint<M: Modulo> {
        val: u32,
        _p: PhantomData<fn() -> M>,
    }
    impl<M: Modulo> Mint<M> {
        #[inline]
        pub fn new(val: u64) -> Self { Mint { val: (val % M::MOD as u64) as u32, _p: PhantomData } }
        pub fn new_signed(val: i64) -> Self { Mint { val: val.rem_euclid(M::MOD as i64) as u32, _p: PhantomData } }
        #[inline]
        pub fn raw(val: u32) -> Self {
            debug_assert!(val < M::MOD);
            Mint { val, _p: marker::PhantomData }
        }
        #[inline]
        pub fn zero() -> Self { Mint { val: 0, _p: marker::PhantomData } }
        #[inline]
        pub fn one() -> Self { Mint { val: 1, _p: marker::PhantomData } }
        #[inline]
        pub fn modulo() -> u32 { M::MOD }
        #[inline]
        pub fn val(&self) -> u32 { self.val }
        pub fn pow(&self, mut exp: u32) -> Self {
            let (mut val, mut res) = (self.val as u64, 1);
            while exp > 0 {
                if exp & 1 == 1 {
                    res = (res * val) % M::MOD as u64;
                }
                val = (val * val) % M::MOD as u64;
                exp >>= 1;
            }
            Self { val: res as u32, _p: PhantomData }
        }
        #[inline]
        pub fn inv(&self) -> Self { self.pow(M::MOD - 2) }
        #[inline]
        pub fn nth_root(n: u32) -> Self {
            debug_assert!(n == 1 << n.trailing_zeros());
            Mint::raw(M::PRIM_ROOT).pow(M::MOD - 1 + (M::MOD - 1) / n)
        }
        #[inline]
        pub fn add_raw(&self, rhs: u32) -> Self {
            debug_assert!(rhs < M::MOD);
            let res = self.val + rhs;
            Mint::raw(if res >= M::MOD { res - M::MOD } else { res })
        }
        #[inline]
        pub fn sub_raw(&self, rhs: u32) -> Self {
            debug_assert!(rhs < M::MOD);
            let (res, f) = self.val.overflowing_sub(rhs);
            Mint::raw(if f { res.wrapping_add(M::MOD) } else { res })
        }
        #[inline]
        pub fn mul_raw(&self, rhs: u32) -> Self {
            debug_assert!(rhs < M::MOD);
            Mint::new(self.val as u64 * rhs as u64)
        }
        #[inline]
        pub fn div_raw(&self, rhs: u32) -> Self {
            debug_assert!(rhs < M::MOD);
            *self / Mint::raw(rhs)
        }
    }
    impl<M: Modulo> Default for Mint<M> {
        fn default() -> Self { Mint::zero() }
    }
    impl<M: Modulo> std::fmt::Debug for Mint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) }
    }
    impl<M: Modulo> std::fmt::Display for Mint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) }
    }
    impl<M: Modulo> Add for Mint<M> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output { self.add_raw(rhs.val) }
    }
    impl<M: Modulo> AddAssign for Mint<M> {
        fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
    }
    impl<M: Modulo> Sub for Mint<M> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output { self.sub_raw(rhs.val) }
    }
    impl<M: Modulo> SubAssign for Mint<M> {
        fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
    }
    impl<M: Modulo> Mul for Mint<M> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output { self.mul_raw(rhs.val) }
    }
    impl<M: Modulo> MulAssign for Mint<M> {
        fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
    }
    impl<M: Modulo> Div for Mint<M> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            debug_assert!(rhs.val != 0);
            self * rhs.inv()
        }
    }
    impl<M: Modulo> DivAssign for Mint<M> {
        fn div_assign(&mut self, rhs: Self) {
            debug_assert!(rhs.val != 0);
            *self *= rhs.inv()
        }
    }
    pub fn combination<M: Modulo>(size: u32) -> impl Fn(usize, usize) -> Mint<M> {
        let mut fact = vec![Mint::<M>::one()];
        fact.append(
            &mut (1..=size)
                .scan(Mint::<M>::one(), |s, v| {
                    *s *= Mint::new(v as u64);
                    Some(*s)
                })
                .collect(),
        );
        let inv = fact[size as usize].inv();
        let mut ifact = vec![inv];
        ifact.append(
            &mut (1..=size)
                .rev()
                .scan(inv, |s, v| {
                    *s *= Mint::new(v as u64);
                    Some(*s)
                })
                .collect(),
        );
        ifact.reverse();
        move |n: usize, k: usize| {
            if n < k {
                Mint::zero()
            } else {
                fact[n] * ifact[k] * ifact[n - k]
            }
        }
    }
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// MontgomeryModint
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // t <- MR(T) = (T + (TN' mod R) * N) / R
    //  if t >= N then return t - N else return t
    //      T := a (0 <= T < NR)
    //      N := modulo()
    //      N':= montgomery_constant_modulo_inv()
    //      R := montgomery_constant_r()
    fn montgomery_reduction(val: u32, modulo: u32, mod_inv: u32) -> u32 {
        let t = ((val as u64).wrapping_add((val.wrapping_mul(mod_inv) as u64).wrapping_mul(modulo as u64)) >> 32) as u32;
        let res = if t >= modulo { t - modulo } else { t };
        res
    }
    fn montgomery_multiplication(lhs: u32, rhs: u32, modulo: u32, mod_inv: u32) -> u32 {
        let a = lhs as u64 * rhs as u64;
        let t = (a.wrapping_add(((a as u32).wrapping_mul(mod_inv) as u64).wrapping_mul(modulo as u64)) >> 32) as u32;
        let res = if t >= modulo { t - modulo } else { t };
        res
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct MontgomeryModint<M: Modulo> {
        pub val: u32,
        _phantom: PhantomData<fn() -> M>,
    }
    impl<M: Modulo> MontgomeryModint<M> {
        #[inline]
        pub fn new(val: u32) -> Self { Self::raw(val.rem_euclid(M::MOD)) }
        pub fn raw(val: u32) -> Self {
            let val = montgomery_multiplication(val, M::R2, M::MOD, M::MOD_INV);
            Self { val, _phantom: PhantomData }
        }
        #[inline]
        pub fn from_mont_expr(val: u32) -> Self { Self { val, _phantom: PhantomData } }
        #[inline]
        pub fn val(&self) -> u32 { montgomery_reduction(self.val, M::MOD, M::MOD_INV) }
        #[inline]
        pub fn val_mont_expr(&self) -> u32 { self.val }
        #[inline]
        pub fn one() -> Self { Self { val: M::R, _phantom: PhantomData } }
        #[inline]
        pub fn zero() -> Self { Self { val: 0, _phantom: PhantomData } }
        pub fn pow(&self, mut n: u32) -> Self {
            let mut val = self.val;
            let mut res = if (n & 1) != 0 { val } else { M::R };
            n >>= 1;
            while n != 0 {
                val = montgomery_multiplication(val, val, M::MOD, M::MOD_INV);
                if n & 1 != 0 {
                    res = montgomery_multiplication(res, val, M::MOD, M::MOD_INV);
                }
                n >>= 1;
            }
            Self { val: res, _phantom: PhantomData }
        }
        #[inline]
        pub fn nth_root(n: u32) -> Self {
            debug_assert!(n == 1 << n.trailing_zeros());
            MontgomeryModint::<M>::new(M::PRIM_ROOT).pow(M::MOD - 1 + (M::MOD - 1) / n)
        }
        #[inline]
        pub fn inv(&self) -> Self { self.pow(M::MOD - 2) }
    }
    impl<M: Modulo> Add for MontgomeryModint<M> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let (t, fa) = self.val.overflowing_add(rhs.val);
            let (u, fs) = t.overflowing_sub(M::MOD);
            Self { val: if fa || !fs { u } else { t }, _phantom: PhantomData }
        }
    }
    impl<M: Modulo> Sub for MontgomeryModint<M> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            let (val, f) = self.val.overflowing_sub(rhs.val);
            Self {
                val: if f { val.wrapping_add(M::MOD) } else { val },
                _phantom: PhantomData,
            }
        }
    }
    impl<M: Modulo> Mul for MontgomeryModint<M> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                val: montgomery_multiplication(self.val, rhs.val, M::MOD, M::MOD_INV),
                _phantom: PhantomData,
            }
        }
    }
    impl<M: Modulo> Div for MontgomeryModint<M> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output { self * rhs.inv() }
    }
    impl<M: Modulo> AddAssign for MontgomeryModint<M> {
        fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
    }
    impl<M: Modulo> SubAssign for MontgomeryModint<M> {
        fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
    }
    impl<M: Modulo> MulAssign for MontgomeryModint<M> {
        fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
    }
    impl<M: Modulo> DivAssign for MontgomeryModint<M> {
        fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
    }
    impl<M: Modulo> std::fmt::Debug for MontgomeryModint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val()) }
    }
    impl<M: Modulo> std::fmt::Display for MontgomeryModint<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val()) }
    }
    impl<M: Modulo> From<u32> for MontgomeryModint<M> {
        fn from(value: u32) -> Self { Self::new(value) }
    }
    impl<M: Modulo> From<u64> for MontgomeryModint<M> {
        fn from(value: u64) -> Self { Self::raw(value.rem_euclid(M::MOD as u64) as u32) }
    }
    impl<M: Modulo> From<i32> for MontgomeryModint<M> {
        fn from(value: i32) -> Self { Self::raw(value.rem_euclid(M::MOD as i32) as u32) }
    }
    impl<M: Modulo> From<i64> for MontgomeryModint<M> {
        fn from(value: i64) -> Self { Self::raw(value.rem_euclid(M::MOD as i64) as u32) }
    }
    impl<M: Modulo> FromStr for MontgomeryModint<M> {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let val = s.parse::<i64>()?;
            if 0 <= val && val < M::MOD as i64 {
                Ok(Self::raw(val as u32))
            } else {
                Ok(Self::from(val))
            }
        }
    }
}
