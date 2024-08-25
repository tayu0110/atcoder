use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Bytes; n], t: marker::Bytes}

    let mut res = [0; 2];
    for s in s {
        let f = {
            let mut s = s.clone();
            s.iter_mut().filter(|s| **s == b'?').for_each(|s| *s = b'a');
            s <= t
        };
        let g = {
            let mut s = s.clone();
            s.iter_mut().filter(|s| **s == b'?').for_each(|s| *s = b'z');
            t <= s
        };
        if f && g {
            res[1] += 1;
        } else if f {
            res[0] += 1;
        }
    }

    println!("{}", (res[0]..=res[0] + res[1]).map(|r| r + 1).join(" "))
}
