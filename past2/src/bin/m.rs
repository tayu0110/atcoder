use proconio::*;

const N: usize = 100001;

fn main() {
    input! {d: usize, l: usize, n: usize, c: [usize; d]}

    let mut bucket = vec![vec![0]; N];
    for (i, &c) in c.iter().enumerate() {
        bucket[c].push(i + 1);
        bucket[c].push(i + d + 1);
    }

    let mut cum = vec![vec![]; N];
    for i in 0..N {
        if bucket.len() == 1 {
            continue;
        }
        bucket[i].sort_unstable();
        cum[i].resize(0, bucket[i].len());
        for j in 1..bucket[i].len() {
            cum[i][j] = cum[i][j - 1] + (bucket[i][j] - bucket[i][j - 1] - 1) / l + 1;
        }
    }

    for _ in 0..n {
        input! {k: usize, f: usize, mut t: usize}

        if bucket[k - 1].len() == 1 {
            println!("0");
            continue;
        }

        let pos = bucket[k - 1].partition_point(|&b| b < f);
        if bucket[k - 1][pos] != f {
            t = t.saturating_sub((bucket[k - 1][pos] - f) / l);
        }
        if t == 0 {
            println!("0");
            continue;
        }
    }
}
