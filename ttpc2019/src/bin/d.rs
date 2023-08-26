use proconio::*;
use std::collections::HashSet;

const MAX: usize = 1000100;

fn main() {
    input! {n: usize, x: [usize; n]}

    let mut sieve = vec![std::usize::MAX; MAX];
    for i in 2..MAX {
        if sieve[i] == std::usize::MAX {
            for j in (2..).take_while(|&j| i * j < MAX) {
                sieve[i * j] = i;
            }
        }
    }

    let primes = sieve
        .into_iter()
        .enumerate()
        .filter(|&(i, v)| v == std::usize::MAX && i >= 2)
        .map(|(i, _)| i)
        .collect::<HashSet<_>>();

    if x.into_iter().fold(0, |s, v| {
        s ^ if v == 7 {
            3
        } else if v == 2 || !primes.contains(&(v - 2)) {
            1
        } else {
            2
        }
    }) == 0
    {
        println!("Ai")
    } else {
        println!("An")
    }
}
