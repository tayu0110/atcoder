use proconio::input;
use proconio::marker::Chars;

fn transpose(h: usize, w: usize, s: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![];
    for j in 0..w {
        let mut v = vec![];
        for i in 0..h {
            v.push(s[i][j]);
        }
        res.push(v);
    }
    res
}

fn main() {
    input! {h: usize, w: usize, s: [Chars; h], t: [Chars; h]}

    let mut s = transpose(h, w, s);
    let mut t = transpose(h, w, t);

    s.sort();
    t.sort();

    if s == t {
        println!("Yes");
    } else {
        println!("No");
    }
}
