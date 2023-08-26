use proconio::input;

fn main() {
    input! {h: usize, w: usize, x: [[usize; w]; h], q: usize, p: [(usize, usize, usize, usize); q]}

    let mut cum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            cum[i + 1][j + 1] = x[i][j];
            cum[i + 1][j + 1] += cum[i + 1][j];
        }
        for j in 1..=w {
            cum[i + 1][j] += cum[i][j];
        }
    }

    for (a, b, c, d) in p {
        println!(
            "{}",
            cum[c][d] + cum[a - 1][b - 1] - cum[a - 1][d] - cum[c][b - 1]
        )
    }
}
