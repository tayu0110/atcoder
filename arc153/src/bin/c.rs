use itertools::Itertools;
use itertools_num::ItertoolsNum;
use proconio::input;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut b = a.iter().rev().cumsum().collect::<Vec<i64>>();
    b.reverse();
    let sum = b.iter().sum::<i64>();

    if b[0] != 0 {
        let mut x = vec![if b[0] < 0 { 1 } else { -1 } * (sum - b[0])];
        for i in 1..n {
            x.push(x[i - 1] + b[0].abs());
        }

        println!("Yes");
        println!("{}", x.iter().join(" "));
    } else if b.iter().all(|&b| b <= 0) || b.iter().all(|&b| b >= 0) {
        println!("No")
    } else {
        let mut y = vec![1; n];

        if sum >= 0 {
            let p = b.iter().position(|&b| b == -1).unwrap();
            y[p] = 1 + sum;
        } else {
            let q = b.iter().position(|&b| b == 1).unwrap();
            y[q] = 1 - sum;
        }

        println!("Yes");
        println!("{}", y.iter().cumsum::<i64>().join(" "))
    }
}
