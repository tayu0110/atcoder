use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}

    if a.iter().sum::<usize>() % n != 0 {
        println!("-1");
        return;
    }

    let mut ret = vec![];
    let ave = a.iter().sum::<usize>() / n;
    let mut t = vec![];
    for i in 0..n {
        if a[i] > ave {
            for j in 0..n {
                if a[i] + a[j] == ave {
                    ret.push((i + 1, j + 1, a[i] - ave));
                    a[i] = ave;
                    a[j] = ave;
                    break;
                }
            }

            if a[i] != ave {
                t.push((i, a[i]));
            }
        }
    }

    let mut buf = vec![vec![]; n];
    for i in 0..n {
        if a[i] < ave {
            for j in 0..1 << t.len() {
                let mut sum = 0;
                for k in 0..t.len() {
                    sum += t[k].1;
                }

                if sum == ave - a[i] {
                    buf[i].push(j);
                }
            }
        }
    }

    let mut memo = vec![usize::MAX; 1 << t.len()];
    memo[0] = 0;
    for i in 0..n {
        if buf[i].len() > 0 {
            for j in 0..1 << t.len() {
                if memo[j] == usize::MAX {
                    continue;
                }

                for k in 0..buf[i].len() {
                    if j & buf[i][k] != 0 {
                        continue;
                    }

                    let new = j | buf[i][k];
                    if memo[new] == usize::MAX || memo[new].count_ones() < memo[j].count_ones() + 1
                    {
                        memo[new] = memo[j] | (1 << i);
                    }
                }
            }
        }
    }

    let max = memo
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (*x != usize::MAX).then_some(x.count_ones() + i.count_ones()))
        .max()
        .unwrap();
    let pos = memo
        .iter()
        .enumerate()
        .rposition(|(i, x)| x.count_ones() + i.count_ones() == max)
        .unwrap();

    println!("{}", ret.len());
    for (x, y, z) in ret {
        println!("{x} {y} {z}");
    }
}
