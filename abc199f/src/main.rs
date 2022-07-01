macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inter!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        #[allow(unused_mut)]
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_ascii_whitespace();
        input_inter!{iter, $($r)*}
    };
}
macro_rules! input_inter {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inter!{$iter $($r)*}
    };
}
macro_rules! read_value {
    ($iter:expr, ( $($t:tt), *)) => {
        ( $(read_value!($iter, $t)), *)
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::fmt;
use std::ops;
const MOD: i64 = 1000000007;
#[derive(Clone, Copy, PartialEq, Eq)]
struct Mint {
    val: i64,
    modulo: i64
}
#[allow(dead_code)]
impl Mint {
    fn new(val: i64) -> Self { let modulo = MOD; Mint { val: val % modulo, modulo } }
    fn pow(&self, mut exp: i64) -> Self {
        let mut val = self.val;
        let mut res = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * val) % self.modulo;
            }
            val = (val * val) % self.modulo;
            exp /= 2;
        }
        Self { val: res, modulo: self.modulo }
    }
    fn inv(&self) -> Self { self.pow(self.modulo - 2) }
}
impl Default for Mint { fn default() -> Self { Mint { val: 0, modulo: MOD } } }
impl fmt::Debug for Mint { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) } }
impl fmt::Display for Mint { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) } }
impl ops::Add for Mint { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self::new(self.val + rhs.val) } }
impl ops::AddAssign for Mint { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl ops::Sub for Mint { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self::new(self.val - rhs.val + self.modulo) } }
impl ops::SubAssign for Mint { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl ops::Mul for Mint { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self::new(self.val * rhs.val) } }
impl ops::MulAssign for Mint { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl ops::Div for Mint { type Output = Self; fn div(self, rhs: Self) -> Self::Output { if rhs.val == 0 { panic!("0 division"); } else { self * rhs.inv() } } }
impl ops::DivAssign for Mint { fn div_assign(&mut self, rhs: Self) { if rhs.val == 0 { panic!("0 division"); } else { *self *= rhs.inv() } } }
struct Combination {
    fact: Vec<Mint>,
    ifact: Vec<Mint>
}
impl Combination {
    #[allow(dead_code)]
    fn new(size: usize) -> Self {
        let mut fact = vec![Mint::new(0); size+1];
        let mut ifact = vec![Mint::new(0); size+1];
        fact[0] = Mint::new(1);
        ifact[0] = Mint::new(1);
        for v in 1..size+1 {
            fact[v] = Mint::new(fact[v-1].val) * Mint::new(v as i64);
        }
        ifact[size] = fact[size].inv();
        for v in 1..size {
            let iv = size as i64 - v as i64;
            ifact[iv as usize] = ifact[iv as usize + 1] * Mint::new(iv);
        }
        Self { fact, ifact }
    }
    #[allow(dead_code)]
    fn get(&self, n: usize, k: usize) -> Mint {
        if n < k {
            Mint::new(0)
        } else {
            self.fact[n] * self.ifact[k] * self.ifact[n-k]
        }
    }
}

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
    input!(n: usize, m: i64, k: usize, a: [i64; n], p: [(usize, usize); m]);
    
    let mut mat = Matrix::id(n);

    let d = Mint::new(m).inv() * Mint::new(2).inv();
    for (mut a, mut b) in p {
        a -= 1;
        b -= 1;
        mat.matrix[a*n+a] -= d;
        mat.matrix[b*n+b] -= d;
        mat.matrix[a*n+b] += d;
        mat.matrix[b*n+a] += d;
    }

    let nm = mat.pow(k);
    for i in 0..n {
        let mut sum = Mint::new(0);
        for j in 0..n {
            sum += nm.get(i, j) * Mint::new(a[j]);
            // sum += nm.matrix[i][j] * Mint::new(a[j]);
        }
        println!("{}", sum);
    }
}
