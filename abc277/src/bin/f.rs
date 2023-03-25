#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn min_except_0(v: &[usize]) -> usize {
    if let Some(min) = v.iter().filter(|c| **c != 0).min() {
        *min
    } else {
        std::usize::MAX
    }
}

fn max_except_0(v: &[usize]) -> usize {
    if let Some(max) = v.iter().filter(|c| **c != 0).max() {
        *max
    } else {
        std::usize::MAX
    }
}

fn transpose(a: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let h = a.len();
    let w = a[0].len();

    let mut res = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            res[w-1-j][i] = a[i][j];
        }
    }

    res
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    if h == 1 || w == 1 {
        println!("Yes");
        return;
    }

    let mut ia = a.iter().enumerate().collect::<Vec<(_, _)>>();
    let mut max = vec![];
    let mut min = vec![];

    for i in 0..h {
        let n = min_except_0(&a[i]);
        let x = max_except_0(&a[i]);
        min.push(n);
        max.push(x);
    }

    ia.sort_by(|&(i, _), &(j, _)| {
        if min[i] != min[j] {
            min[i].cmp(&min[j])
        } else {
            max[i].cmp(&max[j])
        }
    });

    let a = ia.into_iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
    let mut min = vec![];
    let mut max = vec![];
    for i in 0..h {
        min.push(min_except_0(&a[i]));
        max.push(max_except_0(&a[i]));
    }
    for i in 1..h {
        let pmax = max[i-1];
        let min = min[i];

        if min < pmax {
            println!("No");
            return;
        }
    }

    let a = transpose(&a);
    let mut ia = a.into_iter().enumerate().collect::<Vec<(_, _)>>();

    let mut bad = false;

    ia.sort_by(|(i, v), (j, w)| {
        if bad {
            return v[0].cmp(&w[0]);
        }

        let mut decided = false;
        let mut cmp = v[0].cmp(&w[0]);
        for (nv, nw) in v.iter().zip(w.iter()) {
            if *nv != 0 && *nw != 0 && nv != nw {
                if decided {
                    let ncmp = (*nv).cmp(nw);
                    if cmp != ncmp {
                        bad = true;
                        break;
                    }
                } else {
                    decided = true;
                    cmp = (*nv).cmp(nw);
                }
            }
        }

        if !decided {
            i.cmp(&j)
        } else {
            cmp
        }
    });

    if bad {
        println!("No");
        return;
    }

    for (_, v) in ia {
        for v in v.windows(2) {
            if v[0] == 0 || v[1] == 0 {
                continue;
            }

            if v[0] > v[1] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
