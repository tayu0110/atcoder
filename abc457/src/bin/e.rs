use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize); m], q: usize, query: [(usize, usize); q]}
    let mut query = query
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| (r, l, i))
        .collect::<Vec<_>>();
    query.sort_unstable();

    e.sort_unstable_by_key(|e| e.1);
    let mut left = vec![vec![]; n + 1];
    let mut right = vec![vec![]; n + 1];
    for (i, &(l, r)) in e.iter().enumerate() {
        left[l].push((r, i));
        right[r].push((l, i));
    }
    for i in 0..n + 1 {
        left[i].sort_unstable();
        right[i].sort_unstable();
    }

    let mut cur = 0;
    let mut max = (0, 0, 0);
    let mut second = (0, 0, 0);
    let mut ret = vec![false; q];
    for (t, s, i) in query {
        while cur < m && e[cur].1 <= t {
            if max < (e[cur].0, e[cur].1, cur) {
                second = max;
                max = (e[cur].0, e[cur].1, cur);
            } else if second < (e[cur].0, e[cur].1, cur) {
                second = (e[cur].0, e[cur].1, cur);
            }
            cur += 1;
        }
        // eprintln!("i: {i}, s: {s}, t: {t}, max: {max:?}, second: {second:?}");
        let pl = left[s].partition_point(|&l| l.0 <= t);
        if pl > 0 {
            let (r, j) = left[s][pl - 1];
            // eprintln!("r: {r}, j: {j}, i: {i}");
            if r == t {
                if s <= max.0 && max.2 != j {
                    ret[i] = true;
                } else if s <= second.0 && second.2 != j {
                    ret[i] = true;
                }
            } else {
                let mut pr = right[t].partition_point(|&r| r.0 < s);
                while pr < right[t].len() && right[t][pr].1 == j {
                    pr += 1;
                }
                if pr < right[t].len() && right[t][pr].0 <= r + 1 {
                    ret[i] = true;
                }
            }
        } else {
            let pr = right[t].partition_point(|&r| r.0 < s);
            if pr < right[t].len() {
                let (l, j) = right[t][pr];
                if l == s {
                    if s <= max.0 && max.2 != j {
                        ret[i] = true;
                    } else if s <= second.0 && second.2 != j {
                        ret[i] = true;
                    }
                }
            }
        }
    }

    for ret in ret {
        if ret { println!("Yes") } else { println!("No") }
    }
}
