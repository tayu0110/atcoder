use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, _m: usize, s: marker::Chars}

    let mut players = vec![0; n];
    let mut now = 0;
    let mut t = 0;
    for c in s {
        players[now] += 1;
        if c == '+' {
            players[now] += t;
            t = 0;
        } else if c == '-' {
            t += players[now];
            players[now] = 0;
        } else {
        }

        now = (now + 1) % n;
    }

    println!("{}", players.into_iter().join(" "))
}
