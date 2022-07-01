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

const MOD: usize = 1000000007;

fn main() {
    input!(n: usize, a: [usize; n]);

    let mut bit = vec![];
    let mut sum = vec![];
    for i in 0..60 {
        let mut v = vec![];
        for j in 0..n {
            if a[j] & (1usize << i) != 0 {
                v.push(1);
            } else {
                v.push(0);
            }
        }
        bit.push(v.clone());
        for j in 0..n-1 {
            v[j+1] += v[j];
        }
        sum.push(v);
    }

    let mut ans = 0usize;
    for i in 0..60 {
        let mut t = 0usize;
        for j in 0..n {
            let k = sum[i][n-1] - sum[i][j];
            if bit[i][j] == 0 {
                t += k;
            } else {
                t += n - j - 1 - k;
            }
        }
        for _ in 0..i {
            t = t * 2 % MOD;
        }
        ans += t;
        ans %= MOD;
    }
    println!("{}", ans);
}
