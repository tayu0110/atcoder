use proconio::*;

fn main() {
    input! {n: usize, mut a: [[u16; n]; n]}

    let (mut pi, mut pj) = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 0 {
                (pi, pj) = (i, j);
            } else {
                a[i][j] -= 1;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if (pi, pj) == (i, j) {
                continue;
            }
            for k in 0..n {
                if (pi, pj) == (j, k) {
                    continue;
                }

                if a[a[i][j] as usize][k] != a[i][a[j][k] as usize]
                    && a[a[i][j] as usize][k] != 0
                    && a[i][a[j][k] as usize] != 0
                {
                    println!("0");
                    return;
                }
            }
        }
    }

    let mut res = 0;
    'b: for t in 0..n {
        a[pi][pj] = t as u16;
        let (i, j) = (pi, pj);
        for k in 0..n {
            if a[a[i][j] as usize][k] != a[i][a[j][k] as usize] {
                continue 'b;
            }
        }
        let (j, k) = (pi, pj);
        for i in 0..n {
            if a[a[i][j] as usize][k] != a[i][a[j][k] as usize] {
                continue 'b;
            }
        }
        res += 1;
    }

    println!("{res}")
}
