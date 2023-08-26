use proconio::*;

fn rec(
    usable: usize,
    c: &[usize],
    resolved: &mut [bool],
    blocks: &[usize],
    memo: &mut Vec<Vec<Vec<usize>>>,
) -> usize {
    if usable == c.len() {
        if resolved.iter().all(|v| *v) {
            return 0;
        } else {
            return std::usize::MAX;
        }
    }

    let n = resolved.len();
    let mut res = rec(usable + 1, c, resolved, blocks, memo);
    for i in 0..n {
        if resolved[i] {
            continue;
        }

        let mut b = 0;
        for j in 0..=i {
            if !resolved[j] {
                b += blocks[j];
            }
        }

        if memo[i][usable][b] == std::usize::MAX {
            resolved[i] = true;
            let r = rec(usable + 1, c, resolved, blocks, memo);
            resolved[i] = false;

            memo[i][usable][b] = r + b * c[usable];
        }

        res = res.min(memo[i][usable][b]);
    }

    res
}

fn main() {
    input! {n: usize, k: usize, c: [usize; k], mut p: [usize; n]}

    let mut t = vec![];
    {
        let mut p = p.clone();
        while !p.is_empty() {
            let mut v = *p.iter().min().unwrap();
            let mut i = 0;
            let mut w = vec![];
            loop {
                if i == p.len() {
                    break;
                }
                if p[i] == v {
                    w.push(v);
                    v += 1;
                    p.remove(i);
                    if p.is_empty() {
                        break;
                    }
                } else {
                    i += 1;
                }
            }

            t.push(w);
        }
    }

    eprintln!("t: {:?}", t);
    if t.len() == 1 {
        println!("0");
        return;
    }

    t.reverse();
    t.pop().unwrap();
    if t.len() > k {
        println!("-1");
        return;
    }

    let blocks = t.iter().map(|v| v.len()).collect::<Vec<_>>();
    let mut dp = vec![vec![vec![std::usize::MAX; n + 1]; k]; t.len()];
    let mut resolved = vec![false; t.len()];
    let res = rec(0, &c, &mut resolved, &blocks, &mut dp);
    if res == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", res);
    }
    // let mut dp = vec![std::usize::MAX; k + 1];
    // dp[0] = 0;
    // for v in t {
    //     let len = v.len();
    //     let mut new = vec![std::usize::MAX; k + 1];
    //     for i in 0..k {
    //         if dp[i] == std::usize::MAX {
    //             continue;
    //         }
    //         for j in i + 1..=k {
    //             new[j] = new[j].min(dp[i] + len * c[j - 1]);
    //         }
    //     }

    //     dp = new;
    //     eprintln!("{:?}", dp);
    // }

    // let res = *dp.iter().min().unwrap();
    // if res == std::usize::MAX {
    //     println!("-1")
    // } else {
    //     println!("{}", res)
    // }
}
