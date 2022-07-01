use std::collections::HashSet;

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

fn next_permutation<T: Copy + Clone + Eq + Ord>(list: &mut Vec<T>) -> bool {
    let len = list.len();
    for i in (0..len-1).rev() {
        if list[i] < list[i+1] {
            let mut j = list.len() - 1;
            while list[i] > list[j] {
                j -= 1;
            }
            let tmp = list[i];
            list[i] = list[j];
            list[j] = tmp;
            list[i+1..len].sort();
            return true;
        }
        if i == 0 {
            list.reverse();
        }
    }
    false
}

fn main() {
    input!(a: chars, b: chars, c: chars);

    let mut ck = HashSet::new();

    for v in a.iter() {
        ck.insert(*v);
    }
    for v in b.iter() {
        ck.insert(*v);
    }
    for v in c.iter() {
        ck.insert(*v);
    }

    if ck.len() > 10 {
        println!("UNSOLVABLE");
        std::process::exit(0);
    }

    let mut cs = vec![];
    let mut list = (0..10).collect::<Vec<_>>();
    for v in ck {
        cs.push(v);
    }

    loop {
        let mut s = 0usize;
        let mut t = 0usize;
        let mut u = 0usize;
        let mut bad = false;
        for v in &a {
            for i in 0..10 {
                if cs[i] == *v {
                    if s == 0 && list[i] == 0 {
                        bad = true;
                    }
                    s = s * 10 + list[i];
                    break;
                }
            }
        }
        for v in &b {
            for i in 0..10 {
                if cs[i] == *v {
                    if t == 0 && list[i] == 0 {
                        bad = true;
                    }
                    t = t * 10 + list[i];
                    break;
                }
            }
        }
        for v in &c {
            for i in 0..10 {
                if cs[i] == *v {
                    if u == 0 && list[i] == 0 {
                        bad = true;
                    }
                    u = u * 10 + list[i];
                    break;
                }
            }
        }
        if !bad && s + t == u {
            println!("{}", s);
            println!("{}", t);
            println!("{}", u);
            std::process::exit(0);
        }
        if !next_permutation(&mut list) {
            break;
        }
    }
    println!("UNSOLVABLE");
}
