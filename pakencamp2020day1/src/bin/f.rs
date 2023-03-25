#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {p: usize};

    if p == 1 {
        println!("1");
        std::process::exit(0);
    }

    let mut ck = std::collections::HashSet::new();
    let mut ii = 2;
    let (mut f, mut s) = (1, 1);
    ck.insert((f, s));
    loop {
        let iii = ii + 1;
        let t = (f + s) % p;
        if t == 0 {
            println!("{}", iii);
            std::process::exit(0);
        }

        if ck.contains(&(s, t)) {
            println!("-1");
            std::process::exit(0);
        }
        ck.insert((s, t));

        ii = iii;
        f = s;
        s = t;
    }
}
