use proconio::*;

fn main() {
    input! {n: usize}

    let mut t = [[0; 1502]; 1502];
    for _ in 0..n {
        input! {a: usize, b: usize, c: usize, d: usize}
        t[a][b] += 1;
        t[a][d] -= 1;
        t[c][b] -= 1;
        t[c][d] += 1;
    }

    for i in 0..=1500 {
        for j in 0..=1500 {
            t[i][j + 1] += t[i][j];
        }
    }
    for j in 0..=1500 {
        for i in 0..=1500 {
            t[i + 1][j] += t[i][j];
        }
    }

    println!("{}", t.into_iter().flatten().filter(|&t| t > 0).count());
}
