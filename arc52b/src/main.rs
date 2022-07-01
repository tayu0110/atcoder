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

fn main() {
    input!(n: usize, q: usize, s: [(usize, usize, usize); n], t: [(usize, usize); q]);
    const PI: f64 = std::f64::consts::PI;
    const MX: usize = 10101;

    let mut v = vec![0f64; MX];
    for (x, r, h) in s {
        let mut tmp = vec![];
        let mut prev = 0f64;
        for i in 1..h+1 {
            let k = (r * r * i * i * i) as f64 * PI / (3 * h * h) as f64;
            tmp.push(k - prev);
            prev = k;
        }
        for (y, z) in tmp.iter().zip((x+1..x+h+1).rev()) {
            v[z] += *y;
        }
    }
    
    for i in 0..MX-1 {
        v[i+1] += v[i];
    }

    for (a, b) in t {
        println!("{:.15}", v[b] - v[a]);
    }
}
