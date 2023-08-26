use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars, mut t: marker::Chars}

    for i in (1..n).step_by(2) {
        s[i] = (s[i] as u8 ^ b'A' ^ b'B') as char;
        t[i] = (t[i] as u8 ^ b'A' ^ b'B') as char;
    }

    let s = s
        .iter()
        .enumerate()
        .filter(|&c| c.1 == &'A')
        .map(|c| c.0)
        .collect::<Vec<_>>();
    let t = t
        .iter()
        .enumerate()
        .filter(|&c| c.1 == &'A')
        .map(|c| c.0)
        .collect::<Vec<_>>();
    if s.len() != t.len() {
        println!("-1");
        return;
    }

    println!(
        "{}",
        s.into_iter()
            .zip(t)
            .map(|(s, t)| t.max(s) - t.min(s))
            .sum::<usize>()
    );
}
