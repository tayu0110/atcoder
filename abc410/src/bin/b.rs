use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, q: usize, x: [usize; q]}

    let mut b = vec![0; n + 1];
    let mut res = vec![];
    for x in x {
        if x > 0 {
            res.push(x);
            b[x] += 1;
        } else {
            let &min = b[1..].iter().min().unwrap();
            let pos = b.iter().skip(1).position(|&b| b == min).unwrap() + 1;
            res.push(pos);
            b[pos] += 1;
        }
    }

    println!("{}", res.iter().join(" "))
}
