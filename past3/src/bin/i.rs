use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let mut act = vec![(0..n).collect::<Vec<_>>(); 2];
    let mut res = vec![];
    let mut transposed = false;

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {a: usize, b: usize}
            act[transposed as usize].swap(a - 1, b - 1);
        } else if ty == 2 {
            input! {a: usize, b: usize}
            act[1 - transposed as usize].swap(a - 1, b - 1);
        } else if ty == 3 {
            transposed = !transposed;
        } else {
            input! {mut a: usize, mut b: usize}
            if transposed {
                (a, b) = (b, a);
            }
            res.push(n * (act[0][a - 1]) + act[1][b - 1]);
        }
    }

    println!("{}", res.iter().join("\n"))
}
