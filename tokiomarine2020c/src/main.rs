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
    input!(n: usize, k: usize, a: [i64; n]);

    let mut k = k;
    let mut a = a.clone();
    while k > 0 {
        let mut b = vec![0i64; n];
        for (i, v) in a.iter().enumerate() {
            if i as i64 - *v >= 0 {
                b[i - *v as usize] += 1;
            } else {
                b[0] += 1;
            }
            if i + *v as usize + 1 < n {
                b[i + *v as usize + 1] -= 1;
            }
        }
        let mut mx = n + 1;
        for i in 0..n-1 {
            b[i+1] += b[i];
            mx = std::cmp::min(mx, b[i] as usize);
            mx = std::cmp::min(mx, b[i+1] as usize);
        }
        std::mem::swap(&mut a, &mut b);
        if mx == n {
            break;
        }
        k -= 1;
    }

    for v in a {
        println!("{}", v);
    }
}
