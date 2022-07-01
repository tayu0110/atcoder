use proconio::input;

mod flow {
    const INF: usize = 1111222333444555666;
    #[derive(Clone, Copy)]
    struct Edge {
        to: usize,
        cap: usize,
        rev: usize
    }
    impl Edge {
        fn new(to: usize, cap: usize, rev: usize) -> Self {
            Self { to, cap, rev }
        }
    }
    pub struct FordFullkerson {
        size: usize,
        graph: Vec<Vec<Edge>>
    }
    impl FordFullkerson {
        pub fn new(size: usize) -> Self {
            Self { size, graph: vec![vec![]; size] }
        }
        pub fn set_edge(&mut self, from: usize, to: usize, cap: usize) {
            let f_size = self.graph[from].len();
            let t_size = self.graph[to].len();
            self.graph[from].push(Edge::new(to, cap, t_size));
            self.graph[to].push(Edge::new(from, 0, f_size));
        }
        pub fn flow(&mut self, start: usize, goal: usize) -> usize {
            let mut res = 0;
            loop {
                let mut used = vec![false; self.size];
                let f = self.dfs(start, INF, goal, &mut used);
                if f == 0 {
                    return res;
                }
                res += f;
            }
        }
        fn dfs(&mut self, now: usize, f: usize, goal: usize, used: &mut Vec<bool>) -> usize {
            if now == goal {
                return f;
            }
            used[now] = true;
            for i in 0..self.graph[now].len() {
                let Edge { to, cap, rev } = self.graph[now][i];
                if used[to] || cap == 0 {
                    continue;
                }
                let res = self.dfs(to, std::cmp::min(f, cap), goal, used);
                if res == 0 {
                    continue;
                }
                self.graph[to][rev].cap += res;
                self.graph[now][i].cap -= res;
                return res;
            }
            0
        }
    }
}

use flow::FordFullkerson;

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize, usize, usize); n]};

    let p = p.iter().map(|(a, b, c, d)| (*a-1, *b-1, *c, *d)).collect::<Vec<(usize, usize, usize, usize)>>();
    let rid = |i: usize| i;
    let cid = |i: usize| i + h;
    let uid = |i: usize| i + h + w;
    let vid = |i: usize| i + h + w + n;
    let src = h + w + n + n;
    let dest = src + 1;

    let mut flow = FordFullkerson::new(dest + 1);
    for i in 0..h {
        flow.set_edge(src, rid(i), 1);
    }
    for i in 0..w {
        flow.set_edge(cid(i), dest, 1);
    }
    for i in 0..n {
        flow.set_edge(uid(i), vid(i), 1);
    }
    for (i, (a, b, c, d)) in p.iter().enumerate() {
        for j in *a..*c {
            flow.set_edge(rid(j), uid(i), 1);
        }
        for j in *b..*d {
            flow.set_edge(vid(i), cid(j), 1);
        }
    }

    println!("{}", flow.flow(src, dest));
}
