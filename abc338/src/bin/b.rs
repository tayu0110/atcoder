use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut cnt = [0; 256];
    for c in s {
        cnt[c as usize] += 1;
    }

    let &max = cnt.iter().max().unwrap();
    for i in 0..256 {
        if cnt[i] == max {
            println!("{}", (i as u8) as char);
            return;
        }
    }
}
