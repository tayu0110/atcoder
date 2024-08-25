#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize}

    let mut res = "".to_string();
    for k in 1..10 {
        let mut d = vec![0; n];
        let mut now = k;
        for i in 0..n {
            d[i] = now % m;
            now *= 10;
            now %= m;
        }
    
        let mut max = 0;
        for i in 0..n {
            if i > 0 {
                d[i] += d[i-1];
                d[i] %= m;
            }
    
            if d[i] == 0 {
                max = i+1;
            }
        }
    
        if max != 0 {
            let mut s = String::new();
            for _ in 0..max {
                s.push((k as u8 + b'0') as char);
            }

            if s.len() > res.len() {
                res = s;
            } else if s.len() == res.len() && s > res {
                res = s;
            }
        }
    }

    if res.is_empty() {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
