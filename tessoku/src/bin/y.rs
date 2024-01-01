use proconio::*;

fn main() {
    input! {h: usize, w: usize, c: [marker::Chars; h]}

    let mut routes = vec![vec![0u64; w]; h];
    routes[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h && c[i + 1][j] == '.' {
                routes[i + 1][j] += routes[i][j];
            }
            if j + 1 < w && c[i][j + 1] == '.' {
                routes[i][j + 1] += routes[i][j];
            }
        }
    }

    println!("{}", routes[h - 1][w - 1])
}
