use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, a: [Chars; n]}

    let mut bad = false;
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 'D' && a[j][i] != 'D' {
                bad = true;
            }

            if a[i][j] == 'W' && a[j][i] != 'L' {
                bad = true;
            }

            if a[i][j] == 'L' && a[j][i] != 'W' {
                bad = true;
            }
        }
    }

    if bad {
        println!("incorrect")
    } else {
        println!("correct")
    }
}
