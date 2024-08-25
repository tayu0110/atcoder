use proconio::*;

fn main() {
    input! {n: usize, a: [marker::Chars; n], b: [marker::Chars; n]}

    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1)
            }
        }
    }
}
