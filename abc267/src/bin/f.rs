#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};
use std::collections::{
    HashMap, HashSet, VecDeque
};

const INF: usize = 0x3f3f3f3f3f3f3f3f;

fn bfs(root: usize, dist: &mut [usize], t: &[Vec<usize>]) {
    let mut nt = VecDeque::new();
    nt.push_back((root, 0));
    while let Some((now, nd)) = nt.pop_front() {
        if dist[now] != INF {
            continue;
        }
        dist[now] = nd;
        t[now].iter().filter(|to| dist[**to] == INF).for_each(|to| nt.push_back((*to, nd+1)));
    }
}

fn dfs(now: usize, par: usize, depth: usize, max: usize, stack: &mut Vec<usize>, t: &Vec<Vec<usize>>) -> bool {
    if depth == max {
        stack.push(now);
        return true;
    }

    for to in t[now].iter().filter(|to| **to != par) {
        if dfs(*to, now, depth+1, max, stack, t) {
            stack.push(now);
            return true;
        }
    }

    false
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1], q: usize, q: [(usize, usize); q]};

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
    }

    let mut dist = vec![INF; n];
    bfs(0, &mut dist, &t);
    let (_, root) = dist.iter().enumerate().fold((0, 0), |(max, root), (i, now)| { std::cmp::max((max, root), (*now, i)) });
    let mut dist = vec![INF; n];
    bfs(root, &mut dist, &t);
    let (max, root) = dist.iter().enumerate().fold((0, 0), |(max, root), (i, now)| { std::cmp::max((max, root), (*now, i)) });
    let mut dist = vec![INF; n];
    bfs(root, &mut dist, &t);

    let mut stack = vec![];
    dfs(root, INF, 0, max, &mut stack, &t);

    let is_spine = stack.iter().copied().collect::<HashSet<usize>>();

    let mut spine = HashMap::new();
    let mut leaf_dist = vec![INF; n];
    let mut leaf = HashMap::new();
    for v in stack {
        spine.insert(dist[v], v);
        let mut nt = VecDeque::new();
        nt.push_back((v, 0));
        while let Some((now, d)) = nt.pop_front() {
            if leaf_dist[now] == INF {
                leaf_dist[now] = d;
                leaf.insert(now, v);
                for to in &t[now] {
                    if dist[now] < dist[*to] && leaf_dist[*to] == INF && !is_spine.contains(to) {
                        nt.push_back((*to, d+1));
                    }
                }
            }
        }
    }

    let mut doubling = vec![vec![INF; 20]; n];
    for i in 0..20 {
        for j in (0..n).filter(|j| !is_spine.contains(j)) {
            if i == 0 {
                if let Some(to) = t[j].iter().find(|to| dist[**to] < dist[j]) {
                    doubling[j][0] = *to;
                    break;
                }
            } else if doubling[j][i-1] != INF {
                doubling[j][i] = doubling[doubling[j][i-1]][i-1];
            }
        }
    }

    for (u, k) in q {
        let (mut u, mut k) = (u-1, k);
        if leaf_dist[u] > k {
            for i in (0..20).rev() {
                if 1 << i <= k {
                    u = doubling[u][i];
                    k -= 1 << i;
                }
            }
    
            println!("{}", u+1);
            continue;
        }
        k -= leaf_dist[u];
        u = *leaf.get(&u).unwrap();

        if let Some(res) = spine.get(&(k + dist[u])) {
            println!("{}", res + 1);
        } else if dist[u] >= k && spine.contains_key(&(dist[u] - k)) {
            println!("{}", spine.get(&(dist[u] - k)).unwrap() + 1);
        } else {
            println!("-1");
        }
    }
}
