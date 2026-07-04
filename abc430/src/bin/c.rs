use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, s: marker::Bytes}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i];
        if s[i] == b'b' {
            cum[i + 1] += 1;
        }
    }

    let (mut l, mut r) = (0, 0);
    let (mut na, mut nb) = (0, 0);
    let mut ret = 0usize;
    while l < n {
        while r < n && na < a {
            if s[r] == b'a' {
                na += 1;
            } else {
                nb += 1;
            }
            r += 1;
        }

        if na >= a && nb < b {
            let t = cum[r..].partition_point(|&c| nb + c - cum[r] < b);
            // eprintln!("l: {l}, r: {r}, t: {t}");
            ret += t;
        }
        if s[l] == b'a' {
            na -= 1;
        } else {
            nb -= 1;
        }
        l += 1;
    }

    println!("{}", ret);
}
