use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let mut memo = vec![vec![]; 26];
    for (i, c) in s.iter().enumerate() {
        memo[(c - b'a') as usize].push(i);
    }

    let mut ret = 0usize;
    for i in 0..s.len() {
        let mut now = i;
        for &t in &t {
            let pos = memo[(t - b'a') as usize].partition_point(|&t| t < now);
            now = memo[(t - b'a') as usize]
                .get(pos)
                .map(|p| p + 1)
                .unwrap_or(s.len() + 1);
        }

        ret += now - i - 1;
    }

    println!("{ret}")
}
