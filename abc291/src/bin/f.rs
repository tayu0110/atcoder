use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut best_until = vec![std::usize::MAX; n];
    best_until[0] = 0;
    for i in 0..n {
        if best_until[i] == std::usize::MAX {
            continue;
        }

        let best = best_until[i];
        for j in 0..m {
            if s[i][j] == '1' {
                best_until[i + j + 1] = std::cmp::min(best_until[i + j + 1], best + 1);
            }
        }
    }

    let mut best_from = vec![std::usize::MAX; n];
    best_from[n - 1] = 0;
    let mut from = vec![vec![]; n];
    from[n - 1].push((0, std::usize::MAX));
    for i in (0..n).rev() {
        for j in 0..m {
            if s[i][j] == '1' && best_from[i + j + 1] != std::usize::MAX {
                best_from[i] = std::cmp::min(best_from[i + j + 1] + 1, best_from[i]);
                from[i].push((best_from[i + j + 1] + 1, i + j + 1));
            }
        }

        from[i].sort();
        from[i].reverse();
    }

    let mut res = vec![0; n];
    for i in 1..n - 1 {
        let mut min = std::usize::MAX;
        for j in i.saturating_sub(m - 1)..i {
            if from[j].is_empty() {
                continue;
            }

            if best_until[j] == std::usize::MAX {
                continue;
            }

            for k in 0..from[j].len() {
                let (cost, from) = from[j][k];
                if from <= i {
                    continue;
                }

                min = std::cmp::min(min, cost + best_until[j]);
            }
        }

        if min == std::usize::MAX {
            res[i] = -1;
        } else {
            res[i] = min as i32;
        }
    }

    println!("{}", res.into_iter().skip(1).take(n - 2).join(" "))
}
