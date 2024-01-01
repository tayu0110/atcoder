use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; 4*n-1]}

    let mut rem = vec![4; n + 1];
    for a in a {
        rem[a] -= 1;
    }

    let mut res = vec![];
    for (i, v) in rem.into_iter().enumerate().skip(1) {
        for _ in 0..v {
            res.push(i);
        }
    }

    println!("{}", res.iter().join(" "))
}
