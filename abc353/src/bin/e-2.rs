use proconio::*;

fn solve(now: usize, s: &mut [Vec<u8>]) -> usize {
    if s.len() < 2 {
        return 0;
    }
    if s.iter().all(|s| s.len() < now) {
        return s[0].len() * s.len() * (s.len() - 1) / 2;
    }
    s.sort_unstable_by_key(|s| s.get(now).cloned());

    let mut res = 0;
    let mut c = None;
    let mut prev = 0;
    for i in 0..s.len() {
        if s[i].get(now) != c {
            res += now * (i - prev) * (s.len() - i);
            res += solve(now + 1, &mut s[prev..i]);

            c = Some(&s[i][now]);
            prev = i;
        }
    }

    res + solve(now + 1, &mut s[prev..])
}

fn main() {
    input! {n: usize, mut s: [marker::Bytes; n]}

    println!("{}", solve(0, &mut s));
}
