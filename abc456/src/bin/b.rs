use proconio::*;

fn main() {
    input! {a: [[usize; 6]; 3]}

    let mut cnt = 0;
    let t = [
        [4, 5, 6],
        [4, 6, 5],
        [5, 4, 6],
        [5, 6, 4],
        [6, 4, 5],
        [6, 5, 4],
    ];
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                if t.contains(&[a[0][i], a[1][j], a[2][k]]) {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt as f64 / (6 * 6 * 6) as f64);
}
