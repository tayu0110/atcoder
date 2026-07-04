use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let (mut l, mut r, mut u, mut d) = (w, 0, h, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                l = l.min(j);
                r = r.max(j);
                u = u.min(i);
                d = d.max(i);
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if (l..=r).contains(&j) && (u..=d).contains(&i) {
                if s[i][j] == b'.' {
                    println!("No");
                    return;
                }
            } else {
                if s[i][j] == b'#' {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes")
}
