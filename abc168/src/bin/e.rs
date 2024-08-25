// #![allow(unused_imports)]
// use proconio::{
//     input,
//     marker::{Bytes, Chars},
//     *,
// };

// #[derive(Clone, Copy, PartialOrd, Ord, Debug)]
// struct Rational {
//     num: i128,
//     den: i128,
// }

// impl Rational {
//     fn gcd(mut x: i128, mut y: i128) -> i128 {
//         while y != 0 {
//             let (nx, ny) = (y, x % y);
//             x = nx;
//             y = ny;
//         }
//         x
//     }

//     fn new(num: i128, den: i128) -> Self {
//         if num == 0 && den == 0 {
//             return Self { num, den };
//         }
//         if num == 0 {
//             return Self { num, den: 1 };
//         }
//         if den == 0 {
//             return Self { num: 1, den };
//         }
//         let g = Self::gcd(num.abs(), den.abs());
//         let (sn, dn) = (num.signum(), den.signum());
//         let (mut num, den) = (num.abs() / g, den.abs() / g);
//         if sn != 0 {
//             num *= sn;
//         }
//         if dn != 0 {
//             num *= dn;
//         }
//         Self { num, den }
//     }

//     fn inv(&self) -> Self {
//         Self::new(-self.den, self.num)
//     }
// }

// impl PartialEq for Rational {
//     fn eq(&self, other: &Self) -> bool {
//         self.num == other.num && self.den == other.den
//     }
// }

// impl Eq for Rational {}

// type Mint = modint::Mint<modint::Mod1000000007>;

fn main() {
    // input! {n: usize, p: [(i64, i64); n]}

    // let mut memo = std::collections::HashMap::new();
    // for (a, b) in p {
    //     let r = Rational::new(a as i128, b as i128);
    //     *memo.entry(r).or_insert(0) += 1i64;
    // }

    // let mut set = std::collections::HashSet::new();
    // let com = modint::Combination::<modint::Mod1000000007>::new(n + 10);
    // let mut dp = vec![Mint::zero(); 3];
    // dp[2] = Mint::one();
    // for (&k, &v) in &memo {
    //     if set.contains(&k.inv()) {
    //         continue;
    //     }
    //     if k.num == 0 || k.den == 0 {
    //         continue;
    //     }
    //     set.insert(k);

    //     let mut new = vec![Mint::zero(); 3];
    //     let mut sum = Mint::zero();
    //     for i in 1..=v {
    //         sum += com.get(v as usize, i as usize);
    //     }
    //     new[0] = (dp[0] + dp[1] + dp[2]) * sum;

    //     if let Some(&v) = memo.get(&k.inv()) {
    //         let mut sum = Mint::zero();
    //         for i in 1..=v {
    //             sum += com.get(v as usize, i as usize);
    //         }
    //         new[1] = (dp[0] + dp[1] + dp[2]) * sum;
    //     }

    //     new[2] = dp[0] + dp[1] + dp[2];

    //     dp = new;
    // }

    // // eprintln!("memo: {:?}", memo);
    // // eprintln!("dp: {:?}", dp);

    // let mut new = [Mint::zero(); 3];
    // if let Some(&val) = memo.get(&Rational::new(1, 0)) {
    //     let mut sum = Mint::zero();
    //     for i in 1..=val {
    //         sum += com.get(val as usize, i as usize);
    //     }
    //     new[0] = (dp[0] + dp[1] + dp[2]) * sum;
    // }
    // if let Some(&val) = memo.get(&Rational::new(0, 1)) {
    //     let mut sum = Mint::zero();
    //     for i in 1..=val {
    //         sum += com.get(val as usize, i as usize);
    //     }
    //     new[1] += (dp[0] + dp[1] + dp[2]) * sum;
    // }
    // new[2] = dp[0] + dp[1] + dp[2];

    // let mut res = new[0] + new[1] + new[2] - Mint::one();
    // if let Some(&val) = memo.get(&Rational::new(0, 0)) {
    //     res += Mint::raw(val);
    // }
    // println!("{}", res);
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
        fn modulo() -> i64 {
            998_244_353i64
        }
        fn primitive_root() -> i64 {
            3i64
        }
    }
    #[derive(Clone, marker::Copy, PartialEq, Eq)]
    pub enum Mod1000000007 {}
    impl Modulo for Mod1000000007 {
        fn modulo() -> i64 {
            1_000_000_007i64
        }
        fn primitive_root() -> i64 {
            unimplemented!();
        }
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Mint<M>
    where
        M: Modulo,
    {
        val: i64,
        _p: marker::PhantomData<M>,
    }
    impl<M> Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        pub fn new(val: i64) -> Self {
            Mint {
                val: (val % M::modulo() + M::modulo()) % M::modulo(),
                _p: marker::PhantomData,
            }
        }
        pub fn raw(val: i64) -> Self {
            assert!(0 <= val && val < M::modulo());
            Mint {
                val,
                _p: marker::PhantomData,
            }
        }
        pub fn zero() -> Self {
            Mint {
                val: 0i64,
                _p: marker::PhantomData,
            }
        }
        pub fn one() -> Self {
            Mint {
                val: 1i64,
                _p: marker::PhantomData,
            }
        }
        pub fn modulo() -> i64 {
            M::modulo()
        }
        pub fn val(&self) -> i64 {
            self.val
        }
        pub fn pow(&self, mut exp: i64) -> Self {
            let (mut val, mut res) = (self.val, 1);
            while exp > 0 {
                if exp % 2 == 1 {
                    res = (res * val) % M::modulo();
                }
                val = (val * val) % M::modulo();
                exp >>= 1;
            }
            Self {
                val: res,
                _p: marker::PhantomData,
            }
        }
        pub fn inv(&self) -> Self {
            self.pow(M::modulo() - 2)
        }
        pub fn nth_root(n: i64) -> Self {
            assert!(n.abs() == 1 << n.abs().trailing_zeros());
            assert!(M::modulo() - 1 + (M::modulo() - 1) / n >= 0);
            Mint::raw(M::primitive_root()).pow(M::modulo() - 1 + (M::modulo() - 1) / n)
        }
        pub fn add_raw(&self, rhs: i64) -> Self {
            *self + Mint::new(rhs)
        }
        pub fn sub_raw(&self, rhs: i64) -> Self {
            *self - Mint::new(rhs)
        }
        pub fn mul_raw(&self, rhs: i64) -> Self {
            *self * Mint::new(rhs)
        }
        pub fn div_raw(&self, rhs: i64) -> Self {
            *self / Mint::new(rhs)
        }
    }
    impl<M> Default for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn default() -> Self {
            Mint::zero()
        }
    }
    impl<M> std::fmt::Debug for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }
    impl<M> std::fmt::Display for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }
    impl<M> ops::Add for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.val + rhs.val)
        }
    }
    impl<M> ops::AddAssign for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl<M> ops::Sub for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.val - rhs.val + M::modulo())
        }
    }
    impl<M> ops::SubAssign for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }
    impl<M> ops::Mul for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.val * rhs.val)
        }
    }
    impl<M> ops::MulAssign for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }
    impl<M> ops::Div for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        type Output = Self;
        fn div(self, rhs: Self) -> Self::Output {
            assert!(rhs.val != 0);
            self * rhs.inv()
        }
    }
    impl<M> ops::DivAssign for Mint<M>
    where
        M: Modulo + marker::Copy,
    {
        fn div_assign(&mut self, rhs: Self) {
            assert!(rhs.val != 0);
            *self *= rhs.inv()
        }
    }
    pub struct Combination<M>
    where
        M: Modulo,
    {
        fact: Vec<Mint<M>>,
        ifact: Vec<Mint<M>>,
    }
    impl<M> Combination<M>
    where
        M: Modulo + marker::Copy,
    {
        pub fn new(size: usize) -> Self {
            let mut fact = vec![Mint::one(); size + 1];
            let mut ifact = vec![Mint::one(); size + 1];
            let mut buf = vec![Mint::one(); size + 1];
            fact.iter_mut().enumerate().skip(1).for_each(|(i, v)| {
                *v = buf[i - 1] * Mint::raw(i as i64);
                buf[i] = *v;
            });
            ifact[size] = fact[size].inv();
            buf[size] = ifact[size];
            ifact
                .iter_mut()
                .enumerate()
                .skip(1)
                .rev()
                .skip(1)
                .for_each(|(i, v)| {
                    *v = buf[i + 1] * Mint::raw(i as i64 + 1);
                    buf[i] = *v;
                });
            Self { fact, ifact }
        }
        pub fn get(&self, n: usize, k: usize) -> Mint<M> {
            if n < k {
                Mint::zero()
            } else {
                self.fact[n] * self.ifact[k] * self.ifact[n - k]
            }
        }
    }
}
