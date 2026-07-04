use std::fmt::Write as _;

use proconio::*;
use string::RollingHash;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {a: String, b: String}
        if a.len() != b.len() {
            writeln!(buf, "-1").unwrap();
            continue;
        }
        let len = a.len();
        let a = a.repeat(2);
        let hash = RollingHash::new(&a);
        let bhash = RollingHash::new(&b);
        let mut bad = true;
        for i in 0..len {
            if hash.get(i..i + len) == bhash.get(..) {
                writeln!(buf, "{i}").unwrap();
                bad = false;
                break;
            }
        }
        if bad {
            writeln!(buf, "-1").unwrap();
        }
    }

    print!("{buf}")
}
