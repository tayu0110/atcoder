use itertools::Itertools;
use proconio::*;

fn main() {
    input! {x: usize, y: usize}
    let (mut a, mut b) = (x - 2015, y - 2015);
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    let mut res = vec![];
    for i in (1..=a).take_while(|&i| i * i <= a) {
        if a % i == 0 {
            res.push(i);
            if i * i != a {
                res.push(a / i);
            }
        }
    }
    res.sort();

    println!("{}", res.iter().join("\n"))
}
