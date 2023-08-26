use proconio::*;

fn main() {
    input! {n: usize, w: usize, c: usize, p: [(usize, i32, usize); n]}

    let mut t = vec![vec![]; 51];
    for (w, v, c) in p {
        t[c].push((w, v));
    }

    let mut dp = vec![vec![-1; w + 1]; c + 1];
    dp[0][0] = 0;
    for (_k, t) in t.iter().enumerate() {
        if t.is_empty() {
            continue;
        }
        for i in (0..c).rev() {
            let mut new = dp[i].clone();
            let mut updated = false;
            for &(nw, nv) in t {
                for j in (0..w).rev() {
                    if j + nw > w {
                        continue;
                    }
                    if new[j] < 0 {
                        continue;
                    }

                    new[j + nw] = new[j + nw].max(new[j] + nv);
                    updated = true;
                }
            }
            if updated {
                // eprintln!("k: {k}, new: {new:?}");
                dp[i + 1]
                    .iter_mut()
                    .zip(new)
                    .for_each(|(l, r)| *l = (*l).max(r));
                // eprintln!("dp: {dp:?}");
            }
        }
    }
    // eprintln!("dp: {dp:?}");

    println!("{}", dp.iter().flatten().max().unwrap())
}
