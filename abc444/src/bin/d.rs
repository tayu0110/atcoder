use itertools::Itertools;
use proconio::*;

const MAX: usize = 200200;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut buf = vec![0i32; MAX];
    for a in a {
        buf[0] += 1;
        buf[a] -= 1;
    }

    for i in 0..MAX - 1 {
        buf[i + 1] += buf[i];
    }

    let mut ret = vec![];
    for i in 0..MAX - 1 {
        ret.push(buf[i] % 10);
        buf[i + 1] += buf[i] / 10;
    }
    while ret.pop_if(|a| *a == 0).is_some() {}

    println!("{}", ret.iter().rev().join(""));
}
