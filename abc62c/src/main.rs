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

fn solve(h: usize, w: usize) -> usize {
    let mut res = 1usize << 60;

    for v in 1..w {
        let a = h * v;
        let nw = w - v;
        let (b, c) = {
            if h % 2 == 0 || nw % 2 == 0 {
                (h * nw / 2, h * nw / 2)
            } else if h < nw {
                let l = nw / 2;
                let r = nw - nw / 2;
                (l * h, r * h)
            } else {
                let u = h / 2;
                let d = h - h / 2;
                (u * nw, d * nw)
            }
        };
        res = std::cmp::min(res, std::cmp::max(a, std::cmp::max(b, c)) - std::cmp::min(a, std::cmp::min(b, c)));
    }

    res
}

fn main() {
    input!(h: usize, w: usize);

    println!("{}", std::cmp::min(solve(h, w), solve(w, h)));
}
