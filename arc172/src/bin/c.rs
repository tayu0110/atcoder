use proconio::*;

fn main() {
    input! {n: usize, mut c: marker::Bytes}

    if c[0] != b'A' {
        c.iter_mut().for_each(|c| *c ^= b'A' ^ b'B');
    }

    let mut t = vec![0i32; n + 1];
    for i in 0..n {
        if c[i] == b'A' {
            t[i + 1] = t[i] + 1;
        } else {
            t[i + 1] = t[i] - 1;
        }
    }

    t.remove(0);

    let mut res = 1;
    for i in 1..n {
        let a = t[i - 1]
            + match (c[0], c[i]) {
                (b'A', b'B') => -2,
                (b'B', b'A') => 2,
                _ => 0,
            };

        if a.signum() != t[i - 1].signum() {
            res += 1;
        }
    }

    println!("{res}")
}
