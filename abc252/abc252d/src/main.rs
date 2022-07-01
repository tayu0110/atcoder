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
    input!(n: usize, a: [i32; n]);

    let len = a.len();
    let mut a = a.clone();
    a.sort();

    let mut now = a[0];
    let mut cnt = 0;
    let mut t = vec![];
    for v in a {
        if now == v {
            cnt += 1;
        } else {
            t.push(cnt);
            now = v;
            cnt = 1;
        }
    }
    t.push(cnt);

    let k = t.len();
    if k == len {
        println!("{}", n * (n-1) * (n-2) / 6);
        std::process::exit(0);
    }

    let mut sum = 0;
    let mut s = vec![0; k];
    for i in (0..k).rev() {
        s[i] = t[i] * sum;
        sum += t[i];
        if i < k-1 {
            s[i] += s[i+1];
        }
    }

    let mut res = 0usize;
    for i in 0..k-1 {
        res += t[i] * s[i+1];
    }

    println!("{}", res);
}
