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
type Mint = modint::Mint<modint::Mod998244353>;

#[allow(dead_code)]
pub mod convolution {
    const COMPLEX_ONE: num::Complex<f64> = num::Complex::new(1f64, 0f64);
    const PI2: f64 = std::f64::consts::PI * 2f64;
    fn fft(a: &mut Vec<num::Complex<f64>>, inv: bool) {
        let n = a.len();
        let mut r = num::Complex::from_polar(&1f64, &(PI2 / n as f64));
        if inv { r = r.inv(); }
        let b = &mut vec![num::Complex::default(); n];
        for i in (0..n.trailing_zeros()).map(|v| 1usize << v).rev() {
            let z = r.powu(i as u32);
            let mut z2 = COMPLEX_ONE;
            for j in (0..n).step_by(i*2) {
                for k in 0..i {
                    a[i+j+k] *= z2;
                    b[j/2+k] = a[j+k] + a[i+j+k];
                    b[n/2+j/2+k] = a[j+k] - a[i+j+k];
                }
                z2 *= z;
            }
            std::mem::swap(a, b);
        }
        if inv { a.iter_mut().for_each(|now| *now /= num::Complex::new(n as f64, 0f64)); }
    }
    use super::modint;
    type Mint = modint::Mint<modint::Mod998244353>;
    // Only usable 998244353 for modulo.
    fn ntt(a: &mut Vec<Mint>, inv: bool) {
        let n = a.len();
        assert!(n == 1 << n.trailing_zeros());
        if n == 1 { return; }
        let root = Mint::nth_root(n as i64 * if inv { -1 } else { 1 });
        let mut b = vec![Mint::zero(); n];
        let mut roots = vec![Mint::one(); n/2+1];
        for i in 0..n/2 {
            roots[i+1] = roots[i] * root;
        }
        for (w, i) in (0..n.trailing_zeros()).map(|v| 1 << v).zip((0..n.trailing_zeros()).rev().map(|v| 1 << v)) {
            for j in 0..i {
                for k in 0..w {
                    b[k+((w*j)<<1)] = a[k+w*j] + a[k+w*j+(n>>1)];
                    b[k+((w*j)<<1)+w] = roots[w*j] * (a[k+w*j] - a[k+w*j+(n>>1)]);
                }
            }
            std::mem::swap(a, &mut b);
        }
    }
    pub fn convolution(a: &Vec<Mint>, b: &Vec<Mint>) -> Vec<Mint> {
        let mut n = 1;
        let deg = a.len() + b.len() - 1;
        while n < deg {
            n <<= 1;
        }
        let mut na = a.clone();
        let mut nb = b.clone();
        na.resize(n, Mint::new(0));
        nb.resize(n, Mint::new(0));
        ntt(&mut na, false);
        ntt(&mut nb, false);
        let mut nc = na.into_iter().zip(nb.into_iter()).map(|(l, r)| l * r).collect::<Vec<_>>();
        ntt(&mut nc, true);
        let ninv = Mint::new(n as i64).inv();
        nc[0..deg].into_iter().map(|v| *v * ninv).collect::<Vec<_>>()
    }
    pub fn fconvolution(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
        let deg = a.len() + b.len() - 1;
        let mut n = 1;
        while n < deg { n <<= 1; }
        let mut a2 = vec![num::Complex::default(); n];
        a.into_iter().enumerate().for_each(|(i, v)| a2[i] = num::Complex::new(*v, 0f64));
        let mut b2 = vec![num::Complex::default(); n];
        b.into_iter().enumerate().for_each(|(i, v)| b2[i] = num::Complex::new(*v, 0f64));
        fft(&mut a2, false);
        fft(&mut b2, false);
        let mut c2 = a2.into_iter().zip(b2.into_iter()).map(|(l, r)| l * r).collect::<Vec<_>>();
        fft(&mut c2, true);
        c2[0..deg].into_iter().map(|v| v.re).collect()
    }
    pub fn iconvolution(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
        let a = a.iter().map(|v| *v as f64).collect::<Vec<_>>();
        let b = b.iter().map(|v| *v as f64).collect::<Vec<_>>();
        fconvolution(&a, &b).into_iter().map(|v| v.round() as i64).collect()
    }
}
	
fn main() {
    input! {n: usize, m: usize, a: [i64; n], b: [i64; m]};
    let a = a.into_iter().map(|v| Mint::raw(v)).collect();
    let b = b.into_iter().map(|v| Mint::raw(v)).collect();

    let c = convolution::convolution(&a, &b);

    for (i, v) in c.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v);
    }
    println!("");
}