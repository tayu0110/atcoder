use proconio::input;

fn solve(h: usize, w: usize, c: i64, a: &Vec<Vec<i64>>) -> i64 {
    let mut min = vec![vec![std::i64::MAX; w]; h];
    min[0][0] = a[0][0];

    let mut res = std::i64::MAX;
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            if i > 0 {
                min[i][j] = std::cmp::min(min[i][j], min[i - 1][j]);
            }
            if j > 0 {
                min[i][j] = std::cmp::min(min[i][j], min[i][j - 1]);
            }

            res = std::cmp::min(res, a[i][j] + c * (i + j) as i64 + min[i][j]);
            min[i][j] = std::cmp::min(min[i][j], a[i][j] - c * (i + j) as i64);
        }
    }

    res
}

fn main() {
    input! {h: usize, w: usize, c: i64, mut a: [[i64; w]; h]}

    let mut res = solve(h, w, c, &a);

    a.iter_mut().for_each(|v| v.reverse());

    res = std::cmp::min(res, solve(h, w, c, &a));

    println!("{}", res);
}
