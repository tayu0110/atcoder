use proconio::*;

fn main() {
    input! {n: usize, q: usize, mut s: marker::Bytes}

    let mut res = s.windows(3).filter(|&v| v == b"ABC").count() as i32;
    for _ in 0..q {
        input! {x: usize, c: char}
        let x = x - 1;

        res -= s[x.saturating_sub(3)..(x + 3).min(n)]
            .windows(3)
            .filter(|&v| v == b"ABC")
            .count() as i32;
        s[x] = c as u8;
        res += s[x.saturating_sub(3)..(x + 3).min(n)]
            .windows(3)
            .filter(|&v| v == b"ABC")
            .count() as i32;
        println!("{}", res);
    }
}
