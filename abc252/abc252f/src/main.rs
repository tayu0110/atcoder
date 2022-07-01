use std::{collections::{BinaryHeap}, cmp::Reverse};

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
    input!(n: usize, l: usize, a: [usize; n]);
    
    let mut n = n;
    let mut sum = 0usize;
    let mut nt = BinaryHeap::new();
    for v in a {
        sum += v;
        nt.push(Reverse(v));
    }

    if l - sum > 0 {
        nt.push(Reverse(l-sum));
        n += 1;
    }

    let mut k = 1usize;
    while k*2 <= n {
        k *= 2;
    }
    let mut res = 0;
    while nt.len() > 1 {
        let Reverse(n1) = nt.pop().unwrap();
        let Reverse(n2) = nt.pop().unwrap();

        res += n1 + n2;
        nt.push(Reverse(n1+n2));
    }

    println!("{}", res);
}
