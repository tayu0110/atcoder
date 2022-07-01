use proconio::input;

const MOD: i64 = 1000000007;
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
    input! {n: usize, q: usize, p: [(usize, usize, usize, usize); q]};

    let p = p.iter().map(|(x, y, z, w)| (*x - 1, *y - 1, *z - 1, *w)).collect::<Vec<(usize, usize, usize, usize)>>();

    let mut res = Mint::new(1);
    for i in 0..60 as usize {
        let mut cnt = 0i64;
        for j in 0..(1usize << n) {
            let mut bad = false;
            for (x, y, z, w) in &p {
                let wb = (*w as usize & (1usize << i) as usize) >> i;
                let xb = (j & (1usize << x) as usize) >> x;
                let yb = (j & (1usize << y) as usize) >> y;
                let zb = (j & (1usize << z) as usize) >> z;

                if (xb | yb | zb) != wb {
                    bad = true;
                }
            }
            if !bad {
                cnt += 1;
            }
        }

        res *= Mint::new(cnt);
    }

    println!("{}", res);
}