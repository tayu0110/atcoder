#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {s: Chars, t: Chars};

    if s.len() > t.len() {
        println!("No");
        std::process::exit(0);
    }

    let mut sbuf = vec![];
    let mut prev = '0';
    let mut cnt = 0;
    for v in s {
        if prev == v {
            cnt += 1;
        } else {
            sbuf.push((cnt, prev));
            cnt = 1;
            prev = v;
        }
    }
    sbuf.push((cnt, prev));
    let mut tbuf = vec![];
    prev = '0';
    cnt = 0;
    for v in t {
        if prev == v {
            cnt += 1;
        } else {
            tbuf.push((cnt, prev));
            cnt = 1;
            prev = v;
        }
    }
    tbuf.push((cnt, prev));

    if sbuf.len() != tbuf.len() {
        println!("No");
        std::process::exit(0);
    }

    for ((sc, sv), (tc, tv)) in sbuf.into_iter().zip(tbuf.into_iter()) {
        if sv != tv {
            println!("No");
            std::process::exit(0);
        }
        if sc > tc {
            println!("No");
            std::process::exit(0);
        }
        if sc == 1 && tc != 1 {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
