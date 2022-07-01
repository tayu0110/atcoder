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
    input!(n: usize, p: [(usize, usize, usize); n-1]);
    const INF: usize = 1001001001001001001;

    let mut t = vec![vec![]; n];
    for (u, v, w) in p {
        t[u-1].push((v-1, w));
        t[v-1].push((u-1, w));
    }

    let mut d = vec![INF; n];
    let mut nt = VecDeque::new();
    nt.push_back((0, 0));
    while !nt.is_empty() {
        let (now, nd) = nt.pop_back().unwrap();
        if d[now] < INF {
            continue;
        }
        d[now] = nd;
        for (to, w) in &t[now] {
            if d[*to] == INF {
                nt.push_back((*to, nd + w));
            }
        }
    }

    for v in d {
        println!("{}", v % 2);
    }
}
