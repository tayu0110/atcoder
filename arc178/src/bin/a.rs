use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    if a.contains(&n) || a.contains(&1) {
        println!("-1");
    } else {
        let &min = a.iter().min().unwrap();
        let &max = a.iter().max().unwrap();
        print!(
            "{} {} {}",
            (1..min).join(" "),
            (min + 1..=max + 1).join(" "),
            min,
        );

        if max + 2 <= n {
            println!(" {}", (max + 2..=n).join(" "))
        } else {
            println!()
        }
    }
}
