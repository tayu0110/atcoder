use proconio::input;
use proconio::marker::Chars;

fn rec(
    r: usize,
    c: usize,
    h: usize,
    w: usize,
    res: &mut Vec<(usize, usize)>,
    reached: &mut Vec<Vec<bool>>,
    s: &Vec<Vec<char>>,
) -> bool {
    reached[r][c] = true;
    res.push((r, c));

    if res.len() == s.iter().flatten().filter(|&&c| c == '#').count() {
        return true;
    }

    if r > 0 && s[r - 1][c] == '#' && !reached[r - 1][c] {
        if rec(r - 1, c, h, w, res, reached, s) {
            return true;
        }
    }

    if c > 0 && s[r][c - 1] == '#' && !reached[r][c - 1] {
        if rec(r, c - 1, h, w, res, reached, s) {
            return true;
        }
    }

    if r + 1 < h && s[r + 1][c] == '#' && !reached[r + 1][c] {
        if rec(r + 1, c, h, w, res, reached, s) {
            return true;
        }
    }

    if c + 1 < w && s[r][c + 1] == '#' && !reached[r][c + 1] {
        if rec(r, c + 1, h, w, res, reached, s) {
            return true;
        }
    }

    reached[r][c] = false;
    res.pop().unwrap();
    false
}

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '#' {
                continue;
            }

            let mut res = vec![];
            let mut reached = vec![vec![false; w]; h];
            if rec(i, j, h, w, &mut res, &mut reached, &s) {
                println!("{}", res.len());
                for (x, y) in res {
                    println!("{} {}", x + 1, y + 1);
                }
                return;
            }
        }
    }
}
