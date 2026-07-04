use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut pos = (0..n + 1).collect::<Vec<_>>();
    let mut actual = (0..n + 1).collect::<Vec<_>>();
    let mut rev = (0..n + 1).collect::<Vec<_>>();
    let mut res = vec![];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {a: usize, b: usize}
            pos[a] = actual[b];
        } else if ty == 2 {
            input! {a: usize, b: usize}
            actual.swap(a, b);
            rev[actual[a]] = a;
            rev[actual[b]] = b;
        } else {
            input! {a: usize}
            res.push(rev[pos[a]]);
        }
    }

    println!("{}", res.iter().join("\n"))
}
