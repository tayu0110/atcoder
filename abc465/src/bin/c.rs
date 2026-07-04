use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut ret = vec![0; n];
    let mut front = 0;
    let mut back = n;
    let mut f = 0;
    for (i, s) in s.into_iter().enumerate().rev() {
        if s == b'o' {
            f = (f + 1) % 2;
        }

        if f % 2 == 1 {
            ret[front] = i + 1;
            front += 1;
        } else {
            back -= 1;
            ret[back] = i + 1;
        }
    }
    println!("{}", ret.iter().join(" "))
}
