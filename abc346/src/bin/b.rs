use proconio::*;

fn main() {
    input! {w: usize, b: usize}

    let s = "wbwbwwbwbwbw".repeat(500).chars().collect::<Vec<_>>();
    for i in 0.. {
        if i + w + b > s.len() {
            break;
        }

        let mut cw = 0;
        let mut cb = 0;
        for j in i..i + w + b {
            if s[j] == 'w' {
                cw += 1;
            } else {
                cb += 1;
            }
        }

        if cw == w && cb == b {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
