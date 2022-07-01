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

fn parse_expr(s: &Vec<char>, l: usize) -> usize {
    if s[l] == '{' {
        let mut next = parse_expr(s, l+1);

        while s[next] != '}' {
            next = parse_expr(s, next+1);
        }

        next+1
    } else {
        for i in l..s.len() {
            if s[i] == ':' || s[i] == ',' || s[i] == '}' {
                return i;
            }
        }

        0
    }
}

fn solve(s: Vec<char>) -> &'static str {
    if s[1] == '}' {
        "dict"
    } else {
        let end = parse_expr(&s, 1);

        if s[end] == ':' {
            "dict"
        } else {
            "set"
        }
    }
}

fn main() {
    input!(s: chars);

    println!("{}", solve(s));
}
