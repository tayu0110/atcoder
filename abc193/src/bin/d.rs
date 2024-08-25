#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {k: usize, s: Chars, t: Chars}

    let f = |v: [usize; 10]| {
        let mut res = 0;
        for i in 0..10 {
            let mut ten = 1;
            for _ in 0..v[i] {
                ten *= 10;
            }

            res += ten * i;
        }
        res
    };

    let mut tak = s.into_iter().take(4).fold([0usize; 10], |mut s, v| {
        s[v.to_digit(10).unwrap() as usize] += 1;
        s
    });
    let mut aok = t.into_iter().take(4).fold([0usize; 10], |mut s, v| {
        s[v.to_digit(10).unwrap() as usize] += 1;
        s
    });

    let mut res = 0.0;
    for i in 1..10 {
        for j in 1..10 {
            if i == j {
                if tak[i] + aok[j] + 2 > k {
                    continue;
                }
            } else if tak[i] + aok[i] + 1 > k || tak[j] + aok[j] + 1 > k {
                continue;
            }

            tak[i] += 1;
            aok[j] += 1;

            if f(tak) > f(aok) {
                if i == j {
                    res += (k + 2 - tak[i] - aok[j]) as f64 * (k + 1 - tak[i] - aok[j]) as f64
                        / ((9 * k - 8) * (9 * k - 9)) as f64;
                } else {
                    res += (k + 1 - tak[i] - aok[i]) as f64 * (k + 1 - tak[j] - aok[j]) as f64
                        / ((9 * k - 8) * (9 * k - 9)) as f64;
                }
            }

            tak[i] -= 1;
            aok[j] -= 1;
        }
    }

    println!("{}", res);
}
