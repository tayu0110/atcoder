use cpgraph::FixedGraph;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let edges = a
        .iter()
        .enumerate()
        .map(|(i, a)| (i, a - 1))
        .collect::<Vec<_>>();
    let graph = FixedGraph::<(), true>::from((n, edges));
    let scc = graph.scc();

    let mut set = vec![0; n];
    for (i, s) in scc.iter().enumerate() {
        for &s in s {
            set[s] = i;
        }
    }

    let mut t = vec![];
    for (i, s) in scc.iter().enumerate() {
        for &s in s {
            for e in graph.edges(s) {
                if set[e.to()] != i {
                    t.push((i, set[e.to()]));
                }
            }
        }
    }

    t.sort_unstable();
    t.dedup();

    let g = FixedGraph::<(), true>::from((scc.len(), t));
    let topo = g.topological_sort().unwrap();

    let mut memo = vec![usize::MAX; topo.len()];

    fn dfs(now: usize, g: &FixedGraph<(), true>, set: &[usize], memo: &mut [usize]) -> usize {
        if memo[now] != usize::MAX {
            return memo[now] + set[now];
        }

        memo[now] = 0;
        for e in g.edges(now) {
            memo[now] += dfs(e.to(), g, set, memo);
        }

        memo[now] + set[now]
    }

    let mut res = 0;
    let set = scc.iter().map(|s| s.len()).collect::<Vec<usize>>();
    for i in 0..topo.len() {
        let now = topo[i];
        if memo[now] == usize::MAX {
            dfs(now, &g, &set, &mut memo);
        }

        let n = set[now];
        res += n * n;
        res += n * memo[now];
        // eprintln!("now: {now}, n: {n}, res; {res}, scc: {:?}", scc[now]);
    }

    println!("{}", res);
}
