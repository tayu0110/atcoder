use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut cnt = [0; 128];
    for &c in &s {
        cnt[c as usize] += 1;
    }

    let mut res = 0;
    let mut pcnt = [0; 128];
    for c in s {
        cnt[c as usize] -= 1;
        res += (b'A'..=b'Z')
            .map(|c| cnt[c as usize] * pcnt[c as usize])
            .sum::<usize>();
        pcnt[c as usize] += 1;
    }

    println!("{res}")
}
