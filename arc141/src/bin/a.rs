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
    input!(t: usize, p: [usize; t]);

    for n in p {
        let s = n.to_string();
        let len = s.len();
        let mut mx = {
            let mut buf = String::new();
            while buf.len() < len-1 {
                buf.push('9');
            }
            if !buf.is_empty() {
                buf.parse::<usize>().unwrap()
            } else {
                0
            }
        };
        for i in 1..len {
            if len % i == 0 {
                let mut sub = s.as_str()[0..i].to_string();
                let mut buf = String::new();
                while buf.len() < len {
                    buf.push_str(&sub);
                }
                let k = buf.parse::<usize>().unwrap();
                if k <= n {
                    mx = std::cmp::max(mx, k);
                    continue;
                }
                sub = (sub.parse::<usize>().unwrap() - 1).to_string();
                if sub.len() != i {
                    continue;
                }
                buf.clear();
                while buf.len() < len {
                    buf.push_str(&sub);
                }
                let k = buf.parse::<usize>().unwrap();
                if k > n {
                    continue;
                }
                mx = std::cmp::max(mx, k);
            }
        }
        println!("{}", mx);
    }
}
