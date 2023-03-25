use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut res = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                res.push((i as i32, j as i32));
            }
        }
    }

    println!(
        "{}",
        (res[0].0 - res[1].0).abs() + (res[0].1 - res[1].1).abs()
    );
}
