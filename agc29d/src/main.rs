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
    input!(h: usize, w: usize, n: usize, p: [(usize, usize); n]);

    if h == 1 {
        println!("{}", 1);
        std::process::exit(0);
    }

    let mut s = vec![vec![]; w+2];
    for (x, y) in p {
        if x == 2 && y == 1 {
            println!("{}", 1);
            std::process::exit(0);
        }
        s[y].push(x);
    }
    for v in s.iter_mut() {
        if v.len() > 0 {
            v.sort();
        }
        v.push(h+1);
    }

    let mut res = 100100100100100usize;
    let mut ver = 1;
    let mut hor = 1;
    let mut now = 0;
    while ver < h && hor <= w {
        ver += 1;
        now += 1;
        let mut l = -1;
        let mut r = s[hor].len() as i32;
        while r-l > 1 {
            let m = (r + l) / 2;
            if s[hor][m as usize] > ver {
                r = m;
            } else {
                l = m;
            }
        }
        // eprintln!("now: {}, s: {}, ver: {}: hor: {}", now, s[hor][r as usize], ver, hor);
        res = std::cmp::min(res, now + (s[hor][r as usize] - ver) as usize);
        if let Err(_) = s[hor+1].binary_search(&ver) {
            hor += 1;
            if let Ok(_) = s[hor].binary_search(&(ver+1)) {
                res = std::cmp::min(res, now + 1);
                break;
            }
        }
    }

    println!("{}", res);
}
