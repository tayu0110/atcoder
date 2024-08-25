use proconio::*;

fn main() {
    input! {h: usize, w: usize, x: usize, y: usize, s: [marker::Chars; h]}

    let mut res = 0;
    for (dx, dy) in [(0, 1),
        (1, 0),
        (0, 1usize.wrapping_neg()),
        (1usize.wrapping_neg(), 0)] {
        let (mut x, mut y) = (x - 1, y - 1);
        while x < h && y < w && s[x][y] == '.' {
            res += 1;
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
        }
    }

    println!("{}", res - 3)
}
