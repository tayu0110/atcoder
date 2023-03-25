use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize}

    println!("1");
    let mut prev = vec![1];
    for i in 2..=n {
        let mut now = vec![1];
        for j in 1..i - 1 {
            now.push(prev[j - 1] + prev[j]);
        }
        now.push(1);

        println!("{}", now.iter().join(" "));
        prev = now;
    }
}
