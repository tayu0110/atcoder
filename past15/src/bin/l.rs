use proconio::*;

fn solve(a: &mut [Vec<i32>]) -> bool {
    let (h, w) = (a.len(), a[0].len());
    let mut res = [vec![-1; h], vec![-1; w]];
    for i in 0..2 {
        for j in 0..res[i].len() {
            if res[i][j] < 0 {
                let mut nt = vec![[usize::MAX; 2]];
                nt[0][i] = j;
                res[i][j] = 0;
                while let Some([r, c]) = nt.pop() {
                    if r < usize::MAX {
                        for j in 0..w {
                            if a[r][j] >= 0 {
                                a[r][j] ^= res[0][r];
                                if res[1][j] < 0 {
                                    res[1][j] = a[r][j];
                                    nt.push([usize::MAX, j]);
                                }
                            }
                        }
                    } else {
                        for i in 0..h {
                            if a[i][c] >= 0 {
                                a[i][c] ^= res[1][c];
                                if res[0][i] < 0 {
                                    res[0][i] = a[i][c];
                                    nt.push([i, usize::MAX]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    a.iter().flatten().all(|&a| a <= 0)
}

fn main() {
    input! {h: usize, w: usize, mut a: [[i32; w]; h], b: [[i32; w]; h]}

    a.iter_mut()
        .flatten()
        .zip(b.into_iter().flatten())
        .for_each(|(a, b)| {
            if b < 0 {
                *a = -1;
            } else {
                *a ^= b;
            }
        });

    if solve(&mut a) {
        println!("Yes")
    } else {
        println!("No");
    }
}
