use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize}
    let mut res = (1..=n)
        .take_while(|&j| j * j <= n)
        .filter(|j| n % j == 0)
        .flat_map(|j| vec![j, n / j])
        .collect::<Vec<_>>();
    res.sort();
    res.dedup();
    println!("{}", res.iter().join("\n"))
}
