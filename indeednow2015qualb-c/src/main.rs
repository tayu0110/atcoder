use std::collections::{BTreeSet, HashSet};

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
    input!(n: usize, p: [(usize, usize); n-1]);

    let mut t = vec![vec![]; n];

    for (mut a, mut b) in p {
        a -= 1;
        b -= 1;
        t[a].push(b);
        t[b].push(a);
    }

    let mut nt = BTreeSet::new();
    let mut ck = HashSet::new();
    let mut res = vec![];

    nt.insert(0);

    while !nt.is_empty() {
        let now = *nt.iter().next().unwrap();
        nt.remove(&now);
        
        if ck.contains(&now) {
            continue;
        }
        ck.insert(now);

        res.push(now);


        for v in &t[now] {
            if ck.contains(v) {
                continue;
            }
            nt.insert(*v);
        }
    }

    for i in 0..n {
        if i != 0 {
            print!(" ");
        }
        print!("{}", res[i]+1);
    }
    println!();
}
