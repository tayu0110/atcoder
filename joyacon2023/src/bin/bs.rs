use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, s: [Chars; n]}

    for i in 0..n {
        for j in 0..n {
            for (dx, dy) in [(1, 0), (0, 1), (1, 1), (-1, 1)] {
                let (mut w, mut b) = (0, 0);
                let (mut r, mut c) = (i, j);
                for _ in 0..6 {
                    if s[r][c] == '#' {
                        b += 1;
                    } else {
                        w += 1;
                    }

                    let nr = r as i32 + dy;
                    let nc = c as i32 + dx;
                    if nr < 0 || nr == n as i32 || nc < 0 || nc == n as i32 {
                        break;
                    }
                    r = nr as usize;
                    c = nc as usize;
                }

                if w + b != 6 {
                    continue;
                }

                if w <= 2 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No")
}
