use itertools::Itertools;
use proconio::*;

const MAX: usize = 23;
#[target_feature(enable = "avx2")]
unsafe fn fill(i: usize, j: usize, k: usize, p: &mut [[[u8; MAX]; MAX]; MAX], c: &mut [usize; 4]) {
    let mut nc = [0; 4];
    for i in i..i + 7 {
        for j in j..j + 7 {
            for k in k..k + 7 {
                p[i][j][k] += 1;
                nc[p[i][j][k] as usize] += 1;
            }
        }
    }

    c[0] -= nc[1];
    c[1] += nc[1];
    c[1] -= nc[2];
    c[2] += nc[2];
    c[2] -= nc[3];
    c[3] += nc[3];
}
#[target_feature(enable = "avx2")]
unsafe fn unfill(
    i: usize,
    j: usize,
    k: usize,
    p: &mut [[[u8; MAX]; MAX]; MAX],
    c: &mut [usize; 4],
) {
    let mut nc = [0; 4];
    for i in i..i + 7 {
        for j in j..j + 7 {
            for k in k..k + 7 {
                p[i][j][k] -= 1;
                nc[p[i][j][k] as usize] += 1;
            }
        }
    }

    c[0] += nc[0];
    c[1] -= nc[0];
    c[1] += nc[1];
    c[2] -= nc[1];
    c[2] += nc[2];
    c[3] -= nc[2];
}

#[target_feature(enable = "avx2")]
unsafe fn solve(v: Vec<usize>) {
    let mut p = [[[0u8; MAX]; MAX]; MAX];
    let mut c = [0; 4];
    c[0] = MAX.pow(3);
    fill(7, 7, 7, &mut p, &mut c);
    for i in 0..=14 {
        for j in 0..=14 {
            for k in 0..=14 {
                if i > 7 && j > 7 && k > 7 {
                    continue;
                }

                fill(i, j, k, &mut p, &mut c);

                for l in 7..=14 {
                    for m in 7..=14 {
                        for n in 7..=14 {
                            fill(l, m, n, &mut p, &mut c);

                            if &c[1..] == &v {
                                println!("Yes");
                                println!("{}", vec![7, 7, 7, i, j, k, l, m, n].iter().join(" "));
                                return;
                            }

                            unfill(l, m, n, &mut p, &mut c);
                        }
                    }
                }

                unfill(i, j, k, &mut p, &mut c);
            }
        }
    }

    println!("No");
}

fn main() {
    input! {v: [usize; 3]}

    unsafe { solve(v) }
}
