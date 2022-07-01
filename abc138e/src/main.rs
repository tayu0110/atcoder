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
    input!(s: chars, t: chars);

    let mut c = vec![vec![]; 26];
    for (i, v) in s.iter().enumerate() {
        let k = *v as usize - 'a' as usize;
        c[k].push(i);
    }

    let mut now = 0;
    let mut res = 0;

    for v in t {
        let k = v as usize - 'a' as usize;

        let v = &c[k];
        if v.len() == 0 {
            println!("-1");
            std::process::exit(0);
        }
        if *v.last().unwrap() < now {
            res += s.len();
            now = 0;
        }

        let mut l = -1;
        let mut r = v.len() as i32;
        while r-l > 1 {
            let m = (r+l) / 2;
            if v[m as usize] >= now {
                r = m;
            } else {
                l = m;
            }
        }

        now = v[r as usize] + 1;
    }

    println!("{}", res + now);
}
