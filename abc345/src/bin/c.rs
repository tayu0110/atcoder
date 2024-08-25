use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut res = 0usize;
    let mut cnt = [0; 128];
    for (i, c) in s.into_iter().enumerate() {
        res += i - cnt[c as usize];
        cnt[c as usize] += 1;
    }

    if cnt.iter().any(|&c| c > 1) {
        res += 1;
    }

    println!("{}", res)
}
