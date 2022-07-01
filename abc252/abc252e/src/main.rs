use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

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
    input!(n: usize, m: usize, p: [(usize, usize, usize); m]);

    let mut t = vec![vec![]; n];
    let mut cnt = 0;
    for (mut a, mut b, c) in p {
        a -= 1;
        b -= 1;
        t[a].push((b, c, cnt));
        t[b].push((a, c, cnt));
        cnt += 1;
    }

    let mut nt = BinaryHeap::new();
    let mut ck: HashSet<usize> = HashSet::new();
    nt.push(Reverse((0, 0, -1)));
    let mut res = vec![];
    while !nt.is_empty() {
        let Reverse((d, now, idx)) = nt.pop().unwrap();
        if ck.contains(&now) {
            continue;
        }
        ck.insert(now);

        if idx >= 0 {
            res.push(idx);
        }

        for (b, c, cnt) in &t[now] {
            nt.push(Reverse((d+c, *b, *cnt)));
        }
    }

    for i in 0..res.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", res[i]+1);
    }
    println!("");
}
