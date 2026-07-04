use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut height = vec![0; n];
    for (i, &s) in s.iter().enumerate() {
        if s == b'A' {
            height[i + 1] = height[i + 1].max(height[i] + 1);
        }
    }
    for (i, &s) in s.iter().enumerate().rev() {
        if s == b'B' {
            height[i] = height[i].max(height[i + 1] + 1);
        }
    }
    println!("{}", height.into_iter().map(|h| h + 1).sum::<i32>())
}
