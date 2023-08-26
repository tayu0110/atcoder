use proconio::*;

fn rot(a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[n - 1 - j][i] = a[i][j];
        }
    }
    b
}

fn main() {
    input! {n: usize, mut a: [[usize; n]; n], b: [[usize; n]; n]}

    for _ in 0..4 {
        let mut bad = false;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 {
                    if b[i][j] == 0 {
                        bad = true;
                    }
                }
            }
        }

        if !bad {
            println!("Yes");
            return;
        }

        a = rot(a);
    }

    println!("No")
}
