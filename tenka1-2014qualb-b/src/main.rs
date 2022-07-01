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
#[derive(Clone, Copy, PartialEq, Eq)]
struct Mint {
    val: i64,
    modulo: i64
}
#[allow(dead_code)]
impl Mint {
    fn new(val: i64) -> Self { let modulo = 1000000007; Mint { val: val % modulo, modulo } }
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
impl fmt::Debug for Mint { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) } }
impl fmt::Display for Mint { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.val) } }
impl ops::Add for Mint { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self { val: (self.val + rhs.val) % self.modulo, modulo: self.modulo } } }
impl ops::AddAssign for Mint { fn add_assign(&mut self, rhs: Self) { *self = Self { val: (self.val + rhs.val) % self.modulo, modulo: self.modulo } } }
impl ops::Sub for Mint { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self { val: (self.val - rhs.val + self.modulo) % self.modulo, modulo: self.modulo } } }
impl ops::SubAssign for Mint { fn sub_assign(&mut self, rhs: Self) { *self = Self { val: (self.val - rhs.val + self.modulo) % self.modulo, modulo: self.modulo } } }
impl ops::Mul for Mint { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self { val: (self.val * rhs.val) % self.modulo, modulo: self.modulo } } }
impl ops::MulAssign for Mint { fn mul_assign(&mut self, rhs: Self) { *self = Self { val: (self.val * rhs.val) % self.modulo, modulo: self.modulo } } }
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

fn main() {
    input!(n: usize, s: chars, t: [chars; n]);

    let len = s.len();
    let mut dp = vec![Mint::new(0); len+1];
    dp[0] = Mint::new(1);

    for i in 0..len {
        for j in 0..n {
            let ns = &t[j];
            if ns.len() > len - i {
                continue;
            }
            let mut bad = false;
            for k in 0..ns.len() {
                if ns[k] != s[i+k] {
                    bad = true;
                    break;
                }
            }
            if !bad {
                let tmp = dp[i];
                dp[i+ns.len()] += tmp;
            }
        }
    }

    eprintln!("{:?}", dp);

    println!("{}", dp[len]);
}
