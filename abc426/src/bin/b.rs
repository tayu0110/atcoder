use proconio::*;

fn main() {
    input! {s: String}

    let mut cnt = [0; 128];
    for b in s.bytes() {
        cnt[b as usize] += 1;
    }

    println!(
        "{}",
        cnt.iter().position(|&c| c == 1).unwrap() as u8 as char
    )
}
