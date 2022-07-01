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
    input!(n: i64, x: i64);

    let mut res = vec![];
    
    res.push(x);
    if x == (n+1) / 2 {
        for i in 1..n {
            let prev = *res.last().unwrap();
            if i % 2 == 1{
                res.push(prev + i);
            } else {
                res.push(prev - i);
            }
        }
    } else {
        let mut l = vec![];
        let mut r = vec![];
        let mut idx = 0;
        while n / 2 > l.len() as i64 {
            idx += 1;
            if idx == x {
                continue;
            }
            l.push(idx);
        }
        while idx < n {
            idx += 1;
            if idx == x {
                continue;
            }
            r.push(idx);
        }
        // eprintln!("{:?}", l);
        // eprintln!("{:?}", r);
        for i in 0..r.len() {
            res.push(l[l.len()-1-i]);
            res.push(r[i]);
        }
        if l.len() > r.len() {
            res.push(l[0]);
        }
    }

    for v in res {
        println!("{}", v);
    }
}
