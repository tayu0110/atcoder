#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, m: usize}

    let mut check = vec![vec![false; n]; n];

    for _ in 0..m {
        input! {k: usize, x: [usize; k]}

        for i in 0..k {
            for j in 0..k {
                check[x[i]-1][x[j]-1] = true;
            }
        }
    }

    let res = check.iter().flatten().fold(true, |s, v| s & v);
    if res {
        println!("Yes");
    } else {
        println!("No");
    }
}
