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
    input!(n: usize, s: [chars; n]);

    let mut res = 10000000;

    for i in 0..10 {
        let mut dup = 0;
        let mut mx = 0;

        for j in 0..10 {
            let mut d = 0;
            for k in 0..n {
                let v = s[k][j] as usize - '0' as usize;
                if v == i {
                    d += 1;
                }
            }
            if d > 0 {
                if dup < d {
                    dup = d;
                    mx = j;
                } else if dup == d {
                    mx = std::cmp::max(mx, j);
                }
            }
        }

        res = std::cmp::min(res, (dup-1) * 10 + mx);
    }

    println!("{}", res);
}
