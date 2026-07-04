use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::*;

fn dfs(now: usize, node: &[Vec<usize>], child: &[BTreeMap<usize, usize>], ret: &mut Vec<usize>) {
    for (_, &ch) in &child[now] {
        for &n in &node[ch] {
            ret.push(n);
        }
        dfs(ch, node, child, ret);
    }
}

fn main() {
    input! {n: usize, e: [(usize, usize); n]}

    let mut node: Vec<Vec<usize>> = vec![vec![]];
    let mut pos = vec![0; n + 1];
    let mut child = vec![BTreeMap::<usize, usize>::new()];
    for (i, (x, y)) in e.into_iter().enumerate() {
        let p = pos[x];
        if let Some(&np) = child[p].get(&y) {
            node[np].push(i + 1);
            pos[i + 1] = np;
        } else {
            child[p].insert(y, node.len());
            pos[i + 1] = node.len();
            node.push(vec![i + 1]);
            child.push(BTreeMap::new());
        }
    }

    let mut ret = vec![];
    dfs(0, &node, &child, &mut ret);
    println!("{}", ret.iter().join(" "))
}
