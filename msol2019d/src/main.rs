use std::{cmp::Reverse, collections::{VecDeque, HashSet}};

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
    input!(n: usize, p: [(usize, usize); n-1], c: [usize; n]);

    let mut t = vec![vec![]; n];
    for (mut a, mut b) in p {
        a -= 1;
        b -= 1;
        t[a].push(b);
        t[b].push(a);
    }

    let mut tmp = c.iter().map(|x| Reverse(*x)).collect::<Vec<Reverse<usize>>>();
    tmp.sort();
    let mut c = tmp.iter().map(|Reverse(x)| *x).collect::<VecDeque<usize>>();
    let sum = tmp.iter().enumerate().fold(0, |sum, (i, Reverse(x))| if i != 0 { sum + *x } else { sum });
    let mut nt = VecDeque::new();
    nt.push_back(0usize);
    let mut res = vec![0usize; n];
    let mut ck = HashSet::new();

    while !nt.is_empty() {
        let now = nt.pop_front().unwrap();
        if ck.contains(&now) {
            continue;
        }
        ck.insert(now);
        res[now] = c.pop_front().unwrap();

        for to in &t[now] {
            if ck.contains(to) {
                continue;
            }
            nt.push_back(*to);
        }
    }

    println!("{}", sum);
    for v in res {
        println!("{}", v);
    }
}
