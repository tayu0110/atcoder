use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    for i in 0..n {
        for j in 0..n {
            for (dx, dy) in [(0, 1),
                (1, 0),
                (0, !0),
                (!0, 0),
                (1, 1),
                (!0, 1),
                (1, !0),
                (!0, !0)] {
                let (mut r, mut c) = (i, j);
                let mut cnt = 0;
                let mut bad = false;
                for _ in 0..6 {
                    if r >= n || c >= n {
                        bad = true;
                        break;
                    }
                    if s[r][c] == '#' {
                        cnt += 1;
                    }
                    r = r.wrapping_add(dy);
                    c = c.wrapping_add(dx);
                }

                if !bad && cnt >= 4 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
