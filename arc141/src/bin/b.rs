macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

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
    input!(n: usize, m: usize);

    let mut cnt = 0;
    let mut tmp = m;
    while tmp > 0 {
        cnt += 1;
        tmp /= 2;
    }

    if n > cnt {
        println!("0");
        std::process::exit(0);
    }

    let mut l = 1;
    let mut r = 2;
    let mut t = vec![];
    for _ in 0..cnt {
        if r > m {
            t.push(Mint::new((m-l+1) as i64))
        } else {
            t.push(Mint::new((r-l) as i64));
        }
        l *= 2;
        r *= 2;
    }

    let mut dp = vec![vec![Mint::new(0); cnt]; n];
    for (i, v) in dp[0].iter_mut().enumerate() {
        *v = t[i];
    }

    // eprintln!("{:?}", t);

    for i in 0..n-1 {
        for j in 0..cnt {
            let tmp = dp[i][j];
            for k in j+1..cnt {
                dp[i+1][k] += tmp * t[k];
            }
        }
    }

    // eprintln!("{:?}", dp);

    let res = dp[n-1].iter().fold(Mint::new(0), |sum, x| sum + *x);
    println!("{}", res);
}
