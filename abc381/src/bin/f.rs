use proconio::*;

const MAX: usize = 20;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|a| *a -= 1);

    let mut pos = vec![vec![]; MAX];
    for (i, &a) in a.iter().enumerate() {
        pos[a].push(i);
    }

    let mut next = vec![vec![usize::MAX; MAX]; n];
    for i in 0..n {
        for j in 0..MAX {
            let p = pos[j].partition_point(|&p| p <= i);
            if p < pos[j].len() {
                next[i][j] = pos[j][p];
            }
        }
    }

    let mut checked = vec![false; MAX];
    let mut memo = vec![vec![(i8::MAX, usize::MAX); 1 << MAX]; MAX + 1];
    for i in 0..n {
        if !checked[a[i]] {
            checked[a[i]] = true;
            memo[a[i]][1 << a[i]] = (0, i);
        }
    }

    let mut res = 0;
    for i in 0..1 << MAX {
        for j in 0..MAX {
            if memo[j][i] == (i8::MAX, usize::MAX) {
                continue;
            }

            let (d, now) = memo[j][i];
            let nt = next[now][j];
            if nt == usize::MAX {
                continue;
            }
            res = res.max(2 - d);

            for k in 0..MAX {
                if i & (1 << k) == 0 && next[nt][k] < usize::MAX {
                    let next = next[nt][k];
                    let nd = d - 2;
                    let mask = i | (1 << k);
                    memo[k][mask] = memo[k][mask].min((nd, next));
                }
            }
        }
    }

    res = res.max(-memo.iter().flatten().map(|d| d.0).min().unwrap());

    println!("{}", res)
}
