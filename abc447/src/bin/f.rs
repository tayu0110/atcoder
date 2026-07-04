use proconio::*;

fn rec(g: usize, now: usize, par: usize, group: &[usize], t: &[Vec<usize>], memo: &mut [usize]) {
    assert!(group[now] == g);
    for &to in &t[now] {
        if to == par {
            continue;
        }

        if t[to].len() >= 3 {
            memo[to] = memo[now] + 1;
            if t[to].len() >= 4 && group[to] == g {
                rec(g, to, now, group, t, memo);
            }
        }
    }
}

fn grouping(
    g: usize,
    now: usize,
    par: usize,
    t: &[Vec<usize>],
    group: &mut [usize],
    see: &mut Vec<usize>,
) {
    assert!(t[now].len() >= 4);
    for &to in &t[now] {
        if to == par {
            continue;
        }

        if t[to].len() >= 3 {
            assert!(t[to].len() == 3 || group[to] == 0);
            group[to] = g;
            see.push(to);
            if t[to].len() >= 4 {
                grouping(g, to, now, t, group, see);
            }
        }
    }
}

fn solve(
    g: usize,
    mut root: usize,
    t: &[Vec<usize>],
    memo: &mut [usize],
    group: &mut [usize],
) -> usize {
    group[root] = g;
    let mut see = vec![root];
    grouping(g, root, root, t, group, &mut see);
    assert!(see.iter().all(|&b| t[b].len() >= 3 && group[b] == g));
    let mut max = 0;
    for _ in 0..2 {
        memo[root] = 0;
        rec(g, root, root, group, t, memo);
        let new_max = see.iter().map(|&i| memo[i]).max().unwrap();
        assert!(new_max >= max);
        max = new_max;
        root = *see.iter().find(|&&i| memo[i] == max).unwrap();
        assert_eq!(memo[root], max);
        // eprintln!("memo: {memo:?}, root: {root}, max: {max}, see: {see:?}");
    }
    max + 1
}

fn main() {
    input! {q: usize}

    for _ in 0..q {
        input! {n: usize, e: [(usize, usize); n - 1]}

        let mut t = vec![vec![]; n];
        for &(u, v) in &e {
            t[u - 1].push(v - 1);
            t[v - 1].push(u - 1);
        }

        let mut ret = 1;
        for i in 0..n {
            if t[i].len() >= 3 {
                ret = ret.max(1);
                for &to in &t[i] {
                    if t[to].len() >= 3 {
                        ret = ret.max(2);
                    }
                }
            }
        }
        let mut g = 0;
        let mut memo = vec![0; n];
        let mut group = vec![0; n];
        for i in 0..n {
            if t[i].len() >= 4 && group[i] == 0 {
                g += 1;
                ret = ret.max(solve(g, i, &t, &mut memo, &mut group));
            }
        }

        println!("{ret}")
    }
}
