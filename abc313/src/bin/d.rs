use std::io::Write;

use itertools::Itertools;
use proconio::*;

fn main() {
    let mut stdin = source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input! {n: usize, k: usize}

    let mut sum = 0;
    let mut q = vec![0; k + 2];
    for i in 1..=k + 1 {
        println!("? {}", (1..=k + 1).filter(|&j| i != j).join(" "));
        std::io::stdout().flush().unwrap();

        input! {t: usize}

        q[i] = t;
        sum += t;
        sum %= 2;
    }

    let mut res = vec![0; n + 1];
    for i in 1..=k + 1 {
        res[i] = (sum + 2 - q[i]) % 2;
    }

    let sum = res[1..k].iter().sum::<usize>();
    for i in k + 2..=n {
        println!("? {} {}", (1..k).join(" "), i);
        std::io::stdout().flush().unwrap();

        input! {t: usize}
        res[i] = (t + 2 - sum) % 2;
    }

    println!("! {}", res[1..].iter().join(" "))
}
