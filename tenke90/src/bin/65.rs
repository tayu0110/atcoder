use proconio::input;

const MOD: i64 = 998244353;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mint {
    val: i64,
    modulo: i64
}
#[allow(dead_code)]
impl Mint {
    fn new(val: i64) -> Self { let modulo = MOD; Mint { val: val % modulo, modulo } }
    fn val(&self) -> i64 { self.val % self.modulo }
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
impl Default for Mint { fn default() -> Self { Mint { val: 0, modulo: MOD }}}
impl std::fmt::Debug for Mint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
impl std::fmt::Display for Mint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.val) } }
impl std::ops::Add for Mint { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self::new(self.val + rhs.val) } }
impl std::ops::AddAssign for Mint { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Mint { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self::new(self.val - rhs.val + self.modulo) } }
impl std::ops::SubAssign for Mint { fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Mul for Mint { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self::new(self.val * rhs.val) } }
impl std::ops::MulAssign for Mint { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }
impl std::ops::Div for Mint { type Output = Self; fn div(self, rhs: Self) -> Self::Output { if rhs.val == 0 { panic!("0 division"); } else { self * rhs.inv() } } }
impl std::ops::DivAssign for Mint { fn div_assign(&mut self, rhs: Self) { if rhs.val == 0 { panic!("0 division"); } else { *self *= rhs.inv() } } }
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
        for v in (1..size).rev() {
            ifact[v] = ifact[v+1] * Mint::new(v as i64 + 1) ;
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

#[allow(dead_code)]
mod convolution {
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
    use crate::Mint;
    // Only usable 998244353 for modulo.
    // pub type Mint = crate::Mint;
    fn ntt(a: &mut Vec<Mint>, inv: bool) {
        let n = a.len();
        assert!(n == 1 << n.trailing_zeros());
        if n == 1 {
            return;
        }
        let root = if inv {
            Mint::new(3).pow(crate::MOD-1-(crate::MOD-1)/n as i64)
        } else {
            Mint::new(3).pow((crate::MOD-1)/n as i64)
        };
        let mut b = vec![Mint::new(0); n];
        let mut roots = vec![Mint::new(1); n/2+1];
        for i in 0..n/2 {
            roots[i+1] = roots[i] * root;
        }
        let (mut i, mut w) = (n / 2, 1);
        while w < n {
            for j in 0..i {
                for k in 0..w {
                    b[k + ((w * j) << 1)] = a[k + w * j] + a[k + w * j + (n >> 1)];
                    b[k + ((w * j) << 1) + w] = roots[w * j] * (a[k + w * j] - a[k + w * j + (n >> 1)]);
                }
            }
            std::mem::swap(a, &mut b);
            i >>= 1;
            w <<= 1;
        }
    }
    pub fn convolution(a: &Vec<Mint>, b: &Vec<Mint>) -> Vec<Mint> {
        let mut n = 1;
        while n < a.len() + b.len() {
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
        nc.into_iter().map(|v| v * ninv).collect::<Vec<_>>()
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
    const MOD: i64 = 998244353;

    pub fn iconvolution(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
        let a = a.iter().map(|v| *v as f64).collect::<Vec<_>>();
        let b = b.iter().map(|v| *v as f64).collect::<Vec<_>>();
        fconvolution(&a, &b).into_iter().map(|v| v.round() as i64).collect()
    }
}

fn main() {
    input! {r: usize, g: usize, b: usize, k: usize, x: usize, y: usize, z: usize};

    let com = Combination::new(std::cmp::max(r, std::cmp::max(g, b)) + 10);

    let (s, t, u) = (k - y, k - z, k - x);
    let mut res = vec![Mint::new(1)];
    for (r, s) in [(r, s), (g, t), (b, u)].iter() {
        let mut p = vec![Mint::new(0); r + 1];
        for (i, p) in p.iter_mut().enumerate() {
            if i >= *s {
                *p = com.get(*r, i);
            }
        }
        res = convolution::convolution(&mut res, &mut p);
    }
    // eprintln!("{:?}", res);
    let ans = res.get(k).map_or(Mint::new(0), |p| *p);
    println!("{}", ans);
    // let mut res = Mint::new(0);
    // for nr in 0..=r {
    //     for ng in 0..=g {
    //         if nr + ng > x {
    //             break;
    //         }
    //         let nb = k - nr - ng;
    //         if nr + nb > z || ng + nb > y {
    //             continue;
    //         }
    //         res += com.get(r, nr) * com.get(g, ng) * com.get(b, nb);
    //     }
    // }

    // println!("{}", res);
}