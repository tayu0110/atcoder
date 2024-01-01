use proconio::*;

fn main() {
    input! {w: marker::Bytes}

    let mut cnt = [0; 26];
    for w in w {
        cnt[w as usize - b'a' as usize] += 1;
    }

    if cnt.iter().all(|c| c % 2 == 0) {
        println!("Yes")
    } else {
        println!("No")
    }
}
