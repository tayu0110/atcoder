use std::collections::BTreeMap;

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
    input!(s: chars);

    let mut t = vec![];
    let mut d = vec![];
    let mut map = BTreeMap::new();

    let mut diff = 0;
    for c in &s {
        if diff == 0 {
            t.push('g');
            diff += 1;
        } else if *c == 'g' {
            t.push('g');
            diff += 1;
        } else {
            t.push('p');
            diff -= 1;
        }

        d.push(diff);
        *map.entry(diff).or_insert(0) += 1;
    }

    let mut g = 0;
    // eprintln!("{:?}", t);
    // eprintln!("{:?}", d);
    // eprintln!("{:?}", map);

    for (index, v) in d.iter().enumerate() {
        if *v < g || t[index] == 'p' {
            let iter = map.entry(*v).or_insert(0);
            *iter -= 1;
            if *iter == 0 {
                map.remove(v);
            }
            continue;
        }

        let (front, _) = map.iter().next().unwrap();
        // eprintln!("index: {}, v: {}, g: {}, front: {}", index, v, g, front);
        if *front > g+1 {
            t[index] = 'p';
            g += 2;
        }

        let iter = map.entry(*v).or_insert(0);
        *iter -= 1;
        if *iter == 0 {
            map.remove(v);
        }
    }

    // eprintln!("{:?}", t);

    let mut res = 0;
    for i in 0..s.len() {
        if s[i] == 'p' && t[i] == 'g' {
            res -= 1;
        }
        if s[i] == 'g' && t[i] == 'p' {
            res += 1;
        }
    }

    println!("{}", res);
}
