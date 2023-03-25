#[allow(unused_imports)]
use proconio::{input, marker::Bytes, source::line::LineSource};

fn main() {
    input! {mut s: Bytes, mut t: Bytes}

    let s = {
        let mut buf = s.clone();
        while buf.len() < t.len() {
            buf.append(&mut buf.clone());
        }
        buf
    };

    let (slen, tlen) = (s.len(), t.len());
    let mut t = {
        let mut buf = t.clone();
        while buf.len() < slen {
            buf.append(&mut buf.clone());
        }
        buf
    };
    for _ in 0..2 {
        t.append(&mut t.clone());
    }
    for _ in 0..3 {
        t.append(&mut s.clone());
    }

    let mut z = vec![0; t.len()];
    z[0] = t.len();

    let (mut i, mut j) = (1, 0);
    while i < t.len() {
        while i + j < t.len() && t[j] == t[i + j] {
            j += 1;
        }
        z[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < t.len() && k + z[k] < j {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }

    let max = z.into_iter().skip(t.len() - 3 * slen).max().unwrap();

    if max >= 2 * slen {
        println!("-1");
    } else {
        println!("{}", max / tlen);
    }
}
