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
    input! {n: usize, w: usize, a: [usize; n]};

    let mut ff = FordFullkerson::new(n+2);
    
    for (i, v) in a.iter().enumerate() {
        ff.set_edge(n, i, *v);
        ff.set_edge(i, n+1, w);
    }

    for i in 0..n {
        input! {k: usize, c: [usize; k]};
        for v in c {
            ff.set_edge(v-1, i, 0x3f3f3f3f3f3f3f3f);
        }
    }

    let res = ff.flow(n, n+1);
    let ans = a.into_iter().sum::<usize>() - res;
    println!("{}", ans);
}