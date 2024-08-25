use proconio::*;

fn main() {
    input! {s: String}
    println!(
        "{}",
        s.split('*')
            .map(|s| s
                .chars()
                .fold(0, |s, v| (s * 10 + (v as usize - b'0' as usize))
                    % 998244353))
            .fold(1, |s, v| (s * v) % 998244353)
    )
}
