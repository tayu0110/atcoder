use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!(n: usize, m: usize, a: [usize; n]);

    let mut in_d = vec![vec![]; 2*m+1];
    let mut out_d = vec![vec![]; 2*m+1];

    let mut nt = BinaryHeap::new();

    for v in &a {
        let mut cnt = 2;
        while *v * cnt <= 2*m {
            in_d[*v * cnt].push(*v);
            cnt += 1;
        }
    }
    for v in a.iter().rev() {
        for w in &in_d[*v] {
            out_d[*w].push(*v);
        }
        nt.push(Reverse((in_d[*v].len() + out_d[*v].len(), out_d[*v].len(), in_d[*v].len(), *v)));
    }

    let mut ck = HashSet::new();
    while !nt.is_empty() {
        let Reverse((_, _, _, v)) = nt.pop().unwrap();
        let mut bad = false;
        for w in &in_d[v] {
            if ck.contains(w) {
                bad = true;
                break;
            }
        }
        if bad {
            continue;
        }
        for w in &out_d[v] {
            if ck.contains(w) {
                bad = true;
                break;
            }
        }
        if bad {
            continue;
        }
        ck.insert(v);
    }

    eprintln!("{:?}", ck);

    if ck.len() < m {
        for _ in 0..n {
            println!("No");
        }
        std::process::exit(0);
    }

    for v in a {
        if ck.contains(&v) {
            println!("Yes");
            continue;
        }

        let mut cnt = 0;
        for w in &in_d[v] {
            if ck.contains(w) {
                cnt += 1;
            }
        }
        for w in &out_d[v] {
            if ck.contains(w) {
                cnt += 1;
            }
        }

        eprintln!("v: {}, cnt: {}", v, cnt);

        if ck.len() >= cnt && ck.len() - cnt >= m-1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
