use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, s: [String; n], t: [String; m]};

    
    let mut len = 0;
    for v in &s {
        len += v.len();
    }
    
    let mut ck = std::collections::BTreeSet::new();
    for v in t {
        ck.insert(v);
    }
    if n == 1 {
        if ck.contains(&s[0]) || s[0].len() < 3 {
            println!("-1");
        } else {
            println!("{}", s[0]);
        }
        std::process::exit(0);
    }

    let mut t = vec![];
    for i in 0..(1 << (16 - len + n)) {
        if (i as i32).count_ones() as usize == n {
            if i & 1 == 0 {
                continue;
            }
            let mut bad = false;
            for j in 0..16 {
                if i & (0b11 << j) == 0b11 << j {
                    bad = true;
                    break;
                }
            }
            if bad {
                continue;
            }
            t.push(i);
        }
    }

    for v in (0..n).permutations(n) {
        for i in &t {
            let mut buf = String::new();
            let mut now = 0;
            let mut k = *i;
            while k > 0 {
                if k & 1 != 0 {
                    buf.push_str(&s[v[now]]);
                    now += 1;
                } else {
                    buf.push('_');
                }
                k >>= 1;
            }

            if buf.len() < 3 || 16 < buf.len() {
                continue;
            }

            if !ck.contains(&buf) {
                println!("{}", buf);
                std::process::exit(0);
            }
        }
    }

    println!("-1");
}
