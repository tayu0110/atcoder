use proconio::*;
use std::collections::BTreeSet;

fn main() {
    input! {n: usize, s: marker::Chars}

    let mut rem = (0..=n).collect::<BTreeSet<_>>();
    let mut a = 0;
    let mut b = 0;
    for k in 0..n {
        if k % 2 == 0 {
            while a <= n && s[a] == 'A' {
                a += 1;
            }
            rem.remove(&a);
            a += 1;
        } else {
            while b <= n && s[b] == 'B' {
                b += 1;
            }
            rem.remove(&b);
            b += 1;
        }

        let first = *rem.iter().next().unwrap();
        // eprintln!("a: {a}, b: {b}, first: {first}");
        println!("{}", if s[first] == 'A' { "Alice" } else { "Bob" })
    }
}
