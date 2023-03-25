use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [marker::Chars; n], b: [marker::Chars; m]}

    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            let mut res = 0;
            for k in 0..m {
                for l in 0..m {
                    res += (a[i + k][j + l] == b[k][l]) as usize;
                }
            }

            if res == m * m {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
