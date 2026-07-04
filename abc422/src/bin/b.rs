use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'.' {
                continue;
            }

            let mut cnt = 0;
            for (di, dj) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < h && nj < w {
                    cnt += (s[ni][nj] == b'#') as usize;
                }
            }

            if cnt != 4 && cnt != 2 {
                println!("No");
                return;
            }
        }
    }

    println!("Yes")
}
