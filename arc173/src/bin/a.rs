use itertools::Itertools;
use proconio::*;

fn solve(mut k: usize) -> String {
    let mut buf = vec![];
    while k > 0 {
        buf.push((k - 1) % 9 + 1);
        k = (k - 1) / 9;
    }
    buf.reverse();
    // eprintln!("{}", buf.iter().join(""));
    for i in 0..buf.len() - 1 {
        if buf[i] >= buf[i + 1] && buf[i + 1] > 0 {
            buf[i + 1] -= 1;
        }
    }

    buf.iter().join("")
}

fn main() {
    input! {t: usize}

    let mut now = 1;
    for i in 1..=150 {
        while now
            .to_string()
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .any(|v| v[0] == v[1])
        {
            now += 1;
        }
        assert_eq!(solve(i), now.to_string());
        now += 1;
    }

    for _ in 0..t {
        input! {mut k: usize}

        let res = solve(k);
        println!("{}", res)
    }
}
