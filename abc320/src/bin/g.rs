use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut t = vec![vec![VecDeque::new(); 10]; n];
    for i in 0..n {
        for j in 0..m {
            let now = (s[i][j] as u8 - b'0') as usize;
            t[i][now].push_back(j);
        }
    }

    let mut res = usize::MAX;
    for i in 0..10 {
        if t.iter().any(|v| v[i].is_empty()) {
            continue;
        }

        let mut t = t.clone();
        let mut cnt = 0;
        let mut now = 0;
        while !t.is_empty() {
            if cnt > 0 {
                t.iter_mut().for_each(|t| {
                    while *t[i].front().unwrap() <= now {
                        let p = t[i].pop_front().unwrap();
                        t[i].push_back(p + m);
                    }
                });
            }
            let mut min = usize::MAX;
            let mut next = usize::MAX;
            let mut index = usize::MAX;
            for j in 0..t.len() {
                if *t[j][i].front().unwrap() < min {
                    min = t[j][i].pop_front().unwrap();
                    t[j][i].push_back(min + m);
                    next = *t[j][i].front().unwrap();
                    index = j;
                } else if *t[j][i].front().unwrap() == min
                    && t[j][i].len() > 1
                    && next > *t[j][i].iter().nth(1).unwrap()
                {
                    index = j;
                    let l = t[j][i].pop_front().unwrap();
                    t[j][i].push_back(m + l);
                    next = *t[j][i].front().unwrap();
                }
            }

            now = min;
            t.swap_remove(index);
            cnt += 1;
        }
        res = res.min(now);
    }

    println!("{}", res as i64);
}
