#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

const MOD: i64 = 998244353;
#[derive(Clone, Copy, PartialEq, Eq)]
struct Mint {
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

fn main() {
	input! {n: i64};

    if n == 1 {
        println!("0");
        std::process::exit(0);
    }

    let mut p = vec![Mint::new(0); (n*n+1) as usize];
    for i in 1..n*n+1 {
        let mut now = i;
        let mut np = Mint::new(1);
        for _ in 0..n-1 {
            np *= Mint::new(now);
            if now == 0 {
                break;
            }
            now -= 1;
        }
        p[i as usize] = np;
    }

    let mut k = vec![Mint::new(0); (n*n+1) as usize];
    k[0] = Mint::new(1);
    for i in 1..n*n+1 {
        k[i as usize] = k[i as usize - 1] * Mint::new(i);
    }

    let mut res = Mint::new(0);
    for i in 1..(n*n+1) {
        res += p[i as usize - 1] * p[ (n * n - i) as usize] * k[(n*n - 2*n + 1) as usize];
    }
    res *= Mint::new(n*n);

    println!("{}", k[(n*n) as usize] - res);
}
