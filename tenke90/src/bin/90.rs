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

#[derive(Clone)]
struct Matrix {
    row: usize,
    column: usize,
    matrix: Box<[Mint]>
}
#[allow(dead_code)]
impl Matrix {
    fn new(row: usize, column: usize) -> Self { assert!(row > 0 && column > 0); Matrix { row, column, matrix: (vec![Default::default(); row * column]).into_boxed_slice() } }
    fn row(&self) -> usize { self.row }
    fn column(&self) -> usize { self.column }
    fn set(&mut self, row: usize, column: usize, val: Mint) { assert!(row < self.row() && column < self.column()); let c = self.column(); self.matrix[c*row+column] = val; }
    fn get(&self, row: usize, column: usize) -> Mint { assert!(row < self.row() && column < self.column()); self.matrix[row*self.column() + column] }
    fn id(size: usize) -> Self {
        let mut matrix = (vec![Default::default(); size*size]).into_boxed_slice();
        for i in 0..size { matrix[i*size+i] = Mint::new(1); }
        Self { row: size, column: size, matrix }
    }
    fn add(&self, rhs: &Self) -> Self {
        assert!(self.row() == rhs.row() && self.column() == rhs.column());
        let matrix = self.matrix.iter().zip(rhs.matrix.iter()).map(|(x, y)| *x + *y).collect();
        Self { row: self.row(), column: self.column(), matrix }
    }
    fn sub(&self, rhs: &Self) -> Self {
        assert!(self.row() == rhs.row() && self.column() == rhs.column());
        let matrix = self.matrix.iter().zip(rhs.matrix.iter()).map(|(x, y)| *x - *y).collect();
        Self { row: self.row(), column: self.column(), matrix }
    }
    fn mul(&self, rhs: &Self) -> Self {
        unsafe { self.mul_sub(rhs) }
    }
    #[target_feature(enable = "avx2")]
    unsafe fn mul_sub(&self, rhs: &Self) -> Self {
        let (lrow, lcolumn, rrow, rcolumn) = (self.row(), self.column(), rhs.row(), rhs.column());
        assert!(lcolumn == rrow);
        let mut matrix = (vec![Default::default(); lrow*rcolumn]).into_boxed_slice();
        for (s, t) in matrix.chunks_exact_mut(rcolumn).zip(self.matrix.chunks_exact(lcolumn)) {
            for (v, u) in t.iter().zip(rhs.matrix.chunks_exact(rcolumn)) {
                for (x, y) in s.iter_mut().zip(u.iter()) {
                    *x += *v * *y;
                }
            }
        }
        Self { row: lrow, column: rcolumn, matrix }
    }
    fn pow(&self, mut n: usize) -> Self {
        assert!(self.row() == self.column());
        let (mut ret, mut now) = (Self::id(self.row()), self.clone());
        while n > 0 {
            if n % 2 == 1 { ret = ret.mul(&now); }
            now = now.mul(&now);
            n /= 2;
        }
        ret
    }
}

fn main() {
    input! {n: usize, _k: usize};

    let mut t = Matrix::new(2, 2);
    for i in 0..2 {
        for j in 0..2 {
            t.set(i, j, Mint::raw(1 - i as i64 * j as i64));
        }
    }
    let t = t.pow(n);
    let mut r = Matrix::new(2, 1);
    r.set(0, 0, Mint::one());
    r.set(1, 0, Mint::zero());
    let res = t.mul(&r);
    println!("{}", res.get(0, 0) + res.get(1, 0));
}