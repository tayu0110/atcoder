use std::collections::BTreeSet;

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
    input!(n: usize);

    let mut ck = BTreeSet::new();

    if n % 2 == 0 {
        for i in 1..n/2+1 {
            ck.insert((i, n+1-i));
        }
    } else {
        for i in 1..n/2+1 {
            ck.insert((i, n-i));
        }
    }

    let mut res = vec![];
    for i in 1..n+1 {
        for j in i+1..n+1 {
            if ck.contains(&(i, j)) {
                continue;
            }

            res.push((i, j));
        }
    }

    println!("{}", res.len());
    for (v, u) in res {
        println!("{} {}", v, u);
    }
}
