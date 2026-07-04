use proconio::*;

const THRESH: usize = 10;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    if n < THRESH {
        let mut ret = 0;
        'b: for i in 0..1 << n {
            let mut prev = usize::MAX - 1;
            let mut sum = 0;
            for j in 0..n {
                if i & (1 << j) != 0 {
                    if prev + 1 == j {
                        continue 'b;
                    }
                    sum += a[j];
                    prev = j;
                }
            }

            if sum % m == 0 {
                ret += 1;
            }
        }

        println!("{ret}");
        return;
    }

    let mut pre = vec![];
    let mut pre_use_last = vec![];
    let mut stack = vec![vec![]; n / 2];
    for i in 0..n / 2 {
        stack[i].push(0);
    }
    for i in 0..n / 2 {
        while let Some(now) = stack[i].pop() {
            if i == n / 2 - 1 {
                pre_use_last.push((a[i] + now) % m);
            } else {
                pre.push((a[i] + now) % m);
            }
            for j in i + 2..n / 2 {
                stack[j].push(a[i] + now);
            }
        }
    }

    let mut post = vec![];
    let mut post_use_first = vec![];
    let mut stack = vec![vec![]; (n + 1) / 2];
    for i in n / 2..n {
        stack[i - n / 2].push(0);
    }
    for i in (n / 2..n).rev() {
        while let Some(now) = stack[i - n / 2].pop() {
            if i == n / 2 {
                post_use_first.push((a[i] + now) % m);
            } else {
                post.push((a[i] + now) % m);
            }
            for j in n / 2..i - 1 {
                stack[j - n / 2].push(a[i] + now);
            }
        }
    }

    pre.sort_unstable();
    let mut cnt = vec![];
    for p in pre {
        match cnt.last_mut() {
            Some((pre, cnt)) if *pre == p => *cnt += 1,
            _ => cnt.push((p, 1usize)),
        }
    }

    let mut cnt_use_last = vec![];
    pre_use_last.sort_unstable();
    for p in pre_use_last {
        match cnt_use_last.last_mut() {
            Some((pre, cnt)) if *pre == p => *cnt += 1,
            _ => cnt_use_last.push((p, 1usize)),
        }
    }

    // eprintln!("cnt: {cnt:?}, cnt_use_last: {cnt_use_last:?}");
    // eprintln!("post: {post:?}, post_use_first: {post_use_first:?}");

    let mut ret = 1;
    for p in post {
        if let Ok(pos) = cnt.binary_search_by_key(&((m - p) % m), |k| k.0) {
            ret += cnt[pos].1;
        }
        if let Ok(pos) = cnt_use_last.binary_search_by_key(&((m - p) % m), |k| k.0) {
            ret += cnt_use_last[pos].1;
        }
        if p == 0 {
            ret += 1;
        }
    }
    for p in post_use_first {
        if let Ok(pos) = cnt.binary_search_by_key(&((m - p) % m), |k| k.0) {
            ret += cnt[pos].1;
        }
        if p == 0 {
            ret += 1;
        }
    }
    if cnt[0].0 == 0 {
        ret += cnt[0].1;
    }
    if cnt_use_last[0].0 == 0 {
        ret += cnt_use_last[0].1;
    }

    println!("{ret}")
}
