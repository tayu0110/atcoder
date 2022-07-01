use std::collections::VecDeque;

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
    input!(n: usize, s: chars);

    let mut nt = VecDeque::new();

    for (i, v) in s.iter().enumerate() {
        if nt.is_empty() {
            nt.push_back(*v);
        } else if nt.len() == 1 {
            if nt.front().unwrap() == v {
                nt.pop_front().unwrap();
            } else {
                nt.push_back(*v);
            }
        } else {
            if nt.front().unwrap() == v {
                nt.pop_front().unwrap();
            } else if nt.back().unwrap() == v {
                nt.pop_back().unwrap();
            } else if i == n-1 {
                nt.push_back(*v);
            } else {
                let u = s[i+1];
                if *v == u {
                    nt.push_back(*v);
                } else if nt.front().unwrap() == &u {
                    nt.push_back(*v);
                } else {
                    nt.push_front(*v);
                }
            }
        }
    }

    println!("{}", nt.len());
}
