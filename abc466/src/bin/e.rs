use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut e: [(i64, i64); n]}

    for _ in 0..k {
        let mut cum = vec![0; n + 1];
        for (i, &(a, b)) in e.iter().enumerate() {
            cum[i + 1] = cum[i] + b - a;
        }

        let mut min = i64::MAX;
        let mut min_pos = 0;
        let mut max = 0;
        let mut best = (usize::MAX, usize::MAX);
        for (i, &c) in cum.iter().enumerate() {
            if max < c - min {
                max = c - min;
                best = (min_pos, i);
            }
            if min > c {
                min = c;
                min_pos = i;
            }
        }

        if best != (usize::MAX, usize::MAX) {
            let (l, r) = best;
            for i in l..r {
                e[i] = (e[i].1, e[i].0);
            }
        }
    }

    println!("{}", e.into_iter().map(|e| e.0).sum::<i64>());
}
