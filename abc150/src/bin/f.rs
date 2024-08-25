#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

// #[fastout]
fn main() {
    input! {n: usize, a: [i64; n], b: [i64; n]}

    let c = {
        let mut buf = vec![];
        for i in 0..n {
            buf.push(a[i] ^ a[(i+1)%n]);
        }
        buf
    };
    let d = {
        let mut buf = vec![];
        for i in 0..n {
            buf.push(b[i] ^ b[(i+1)%n]);
        }
        buf
    };

    let v = d.into_iter().chain(vec![-1]).chain(c.iter().cloned()).chain(c.iter().cloned()).collect::<Vec<_>>();
    let z = zalgorithm(&v);

    let k = z.into_iter().skip(n+1).take(n).enumerate().filter(|(_, k)| *k == n).map(|(i, _)| i).collect::<Vec<_>>();
    let l = k.iter().map(|i| a[*i] ^ b[0]).collect::<Vec<_>>();

    k.into_iter().zip(l).for_each(|(k, l)| { println!("{} {}", k, l); });
}

pub fn zalgorithm(s: &[i64]) -> Vec<usize> {
    let mut z = vec![0; s.len()];
    z[0] = s.len();

    let (mut i, mut j) = (1, 0);
    while i < s.len() {
        while i + j < s.len() && s[j] == s[i+j] {
            j += 1;
        }
        z[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }

        let mut k = 1;
        while i + k < s.len() && k + z[k] < j {
            z[i+k] = z[k];
            k += 1;
        }

        i += k;
        j -= k;
    }

    z
}
