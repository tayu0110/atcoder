use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut map = vec![vec![false; n]; n];
    for (r, c) in p {
        for (dx, dy) in vec![
            (0, 1),
            (1, 0),
            (0, !0),
            (!0, 0),
            (1, 1),
            (!0, !0),
            (1, !0),
            (!0, 1),
        ] {
            let (mut r, mut c) = (r - 1, c - 1);
            while r < n && c < n {
                map[r][c] = true;
                r = r.wrapping_add(dy);
                c = c.wrapping_add(dx);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if !map[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
    println!("-1")
}
