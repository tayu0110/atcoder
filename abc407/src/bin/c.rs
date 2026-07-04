use proconio::*;

fn main() {
    input! {mut s: marker::Bytes}
    s.push(b'0');

    println!(
        "{}",
        s.len() - 1
            + s.windows(2)
                .map(|v| {
                    let s = v[0] - b'0';
                    let t = v[1] - b'0';
                    ((s + 10 - t) % 10) as usize
                })
                .sum::<usize>()
    )
}
