use itertools::Itertools;
use proconio::fastout;
use proconio::input;

fn dfs(now: usize, children: &Vec<Vec<usize>>, cnt: &mut [usize], res: &mut Vec<usize>) {
    if children[now].is_empty() {
        for _ in 0..cnt[now] {
            res.push(now);
        }
        cnt[now] = 0;
        return;
    }

    res.push(now);
    cnt[now] -= 1;
    for &child in &children[now] {
        dfs(child, children, cnt, res);
        res.push(now);
        cnt[now] -= 1;
    }

    for _ in 0..cnt[now] {
        res.push(now);
    }
    cnt[now] = 0;
}

#[fastout]
fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, mut c: [usize; n]}

        let mut cnt = vec![0usize; n + 1];
        let mut max = 0;
        for &c in &c {
            cnt[c] += 1;
            max = std::cmp::max(max, cnt[c]);
        }
        if max == 1 {
            println!("{}", c.iter().join(" "));
            continue;
        }

        let mut rem_one = vec![];
        let mut rem_mul = std::collections::BinaryHeap::new();
        for i in 0..=n {
            if cnt[i] == 1 {
                rem_one.push(i);
            } else if cnt[i] > 1 {
                rem_mul.push(std::cmp::Reverse((cnt[i], i)));
            }
        }
        let mut children = vec![vec![]; n + 1];
        let mut root = 0;
        while let Some(now) = rem_one.pop() {
            if let Some(std::cmp::Reverse((c, i))) = rem_mul.pop() {
                children[i].push(now);
                if c == 2 {
                    rem_one.push(i);
                } else {
                    rem_mul.push(std::cmp::Reverse((c - 1, i)));
                }
                root = i;
            }
        }

        if let Some(std::cmp::Reverse((_, mut prev))) = rem_mul.pop() {
            while let Some(std::cmp::Reverse((_, i))) = rem_mul.pop() {
                children[i].push(prev);
                prev = i;
                root = i;
            }
        }

        let mut res = Vec::with_capacity(n);
        dfs(root, &children, &mut cnt, &mut res);
        for i in 0..=n {
            while cnt[i] > 0 {
                res.push(i);
                cnt[i] -= 1;
            }
        }
        println!("{}", res.iter().join(" "));
    }
}
