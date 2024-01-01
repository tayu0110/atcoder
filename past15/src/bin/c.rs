use itertools::Itertools;
use proconio::*;

fn main() {
    input! {x: i32, y: i32, r: i32, n: i32}
    let mut res = vec![];
    for i in -n..n + 1 {
        let mut buf = vec![];
        for j in -n..n + 1 {
            if (i - x) * (i - x) + (j - y) * (j - y) <= r * r {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        res.push(buf);
    }

    for b in res {
        println!("{}", b.iter().join(" "))
    }
}
