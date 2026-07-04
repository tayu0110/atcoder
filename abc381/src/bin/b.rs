use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    if s.len() % 2 != 0 {
        println!("No");
        return;
    }

    let mut cnt = [0; 256];
    for c in s.chunks_exact(2) {
        if c[0] != c[1] {
            println!("No");
            return;
        }
        cnt[c[0] as usize] += 2;
    }

    if cnt.iter().all(|&c| c == 0 || c == 2) {
        println!("Yes")
    } else {
        println!("No")
    }
}
