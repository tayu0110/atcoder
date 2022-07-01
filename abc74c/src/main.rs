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
    input!(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32);

    let a = a * 100;
    let b = b * 100;

    let mut res_m = 0;
    let mut res_d = 1;
    let mut water = a;
    let mut sugar = 0;

    for i in 0..(f/a+1) {
        let rf = f - i * a;
        for j in 0..(rf/b+1) {
            let rrf = rf - j * b;
            if rrf < 0 {
                break;
            }

            let w = i * a + j * b;
            let mx = std::cmp::min(rrf, w / 100 * e);

            for k in 0..(mx/c+1) {
                let mut ns = k * c;
                ns += (mx - ns) / d * d;

                if res_m * (w + ns) < ns * res_d {
                    res_m = ns;
                    res_d = w + ns;
                    water = w;
                    sugar = ns;
                }
            }

            for k in 0..(mx/d+1) {
                let mut ns = k * d;
                ns += (mx - ns) / c * c;

                if res_m * (w + ns) < ns * res_d {
                    res_m = ns;
                    res_d = w + ns;
                    water = w;
                    sugar = ns;
                }
            }
        }
    }

    println!("{} {}", water+sugar, sugar);
}
