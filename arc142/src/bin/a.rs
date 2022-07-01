use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {n: usize, k: usize};

    if n < k {
        println!("0");
        std::process::exit(0);
    }

    if k % 10 == 0 {
        println!("1");
        std::process::exit(0);
    }

    let mut buf = HashSet::new();

    let mut s = k.to_string();
    for _ in 0..14 {
        if s.len() > 14 {
            break;
        }
        let t = s.parse::<usize>().unwrap();
        buf.insert(t);
        s.push('0');
    }

    let mut s = k.to_string().chars().collect::<Vec<char>>();
    s.reverse();
    let mut s = {
        let mut buf = String::new();
        for v in s {
            buf.push(v);
        }
        buf
    };

    if s.parse::<usize>().unwrap() < k {
        println!("0");
        std::process::exit(0);
    }

    for _ in 0..14 {
        if s.len() > 14 {
            break;
        }
        let t = s.parse::<usize>().unwrap();
        buf.insert(t);
        s.push('0');
    }

    let mut res = 0;
    for v in buf {
        if v <= n {
            res += 1;
        }
    }
    println!("{}", res);
}