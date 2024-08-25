use cpgraph::FixedTree;
use proconio::*;

fn dfs(now: usize, par: usize, d: usize, depth: &mut [usize], t: &FixedTree<(), false>) {
    depth[now] = d;

    for to in t.edges(now).filter(|e| e.to() != par) {
        dfs(to.to(), now, d + 1, depth, t);
    }
}

#[fastout]
fn main() {
    input! {n: usize, s: usize, t: usize, e: [(usize, usize); n - 1]}

    let tree =
        FixedTree::<(), false>::try_from((n, e.into_iter().map(|(s, t)| (s - 1, t - 1)))).unwrap();

    let mut depth = vec![usize::MAX; n];
    dfs(s - 1, s - 1, 0, &mut depth, &tree);
    let lca = tree.lca(s - 1);

    for i in 0..n {
        if s - 1 == i || t - 1 == i {
            println!("1");
            continue;
        }

        let l = lca(t - 1, i);
        if l == t - 1 {
            println!("{}", depth[i] - depth[t - 1] + 1);
        } else if l == s - 1 {
            println!("{}", depth[i] + 1);
        } else {
            println!("{}", depth[i] - depth[l] + 1);
        }
    }
}
