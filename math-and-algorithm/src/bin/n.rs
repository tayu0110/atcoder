use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize}
    let mut res = vec![];
    let mut now = n;
    for i in (2..=n).take_while(|&j| j * j <= n) {
        while now % i == 0 {
            res.push(i);
            now /= i;
        }
    }

    if now != 1 {
        res.push(now);
    }
    res.sort();

    println!("{}", res.iter().join(" "))
}
