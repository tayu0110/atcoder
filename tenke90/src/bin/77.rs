use std::collections::HashMap;

use proconio::input;

mod flow {
    pub const INF: i64 = 0x3f3f3f3f3f3f3f3f;
    #[derive(Clone, Copy)]
    pub struct Edge {
        pub to: usize,
        pub cap: i64,
        pub rev: usize
    }
    impl Edge {
        fn new(to: usize, cap: i64, rev: usize) -> Self { Self { to, cap, rev } }
    }
    pub struct Dinic {
        size: usize,
        level: Vec<i32>,
        iter: Vec<usize>,
        graph: Vec<Vec<Edge>>
    }
    #[allow(dead_code)]
    impl Dinic {
        pub fn new(size: usize) -> Self { Self { size, level: vec![0; size], iter: vec![0; size], graph: vec![vec![]; size] } }
        pub fn set_edge(&mut self, from: usize, to: usize, cap: i64) {
            let (rev_from, rev_to) = (self.graph[to].len(), self.graph[from].len());
            self.graph[from].push(Edge::new(to, cap, rev_from));
            self.graph[to].push(Edge::new(from, 0, rev_to));
        }
        pub fn get_graph(&self) -> &Vec<Vec<Edge>> {
            &self.graph
        }
        fn bfs(&mut self, start: usize) {
            self.level = vec![-1; self.size];
            let mut nt: std::collections::VecDeque<(usize, i32)> = std::collections::VecDeque::new();
            nt.push_back((start, 0i32));
            while let Some((now, nd)) = nt.pop_front() {
                if self.level[now] >= 0 { continue; }
                self.level[now] = nd;
                self.graph[now].iter().for_each(|e| if e.cap > 0 && self.level[e.to] < 0 { nt.push_back((e.to, nd+1)) });
            }
        }
        fn dfs(&mut self, now: usize, target: usize, f: i64) -> i64 {
            if now == target { return f; }
            let mut i = self.iter[now];
            while i < self.graph[now].len() {
                let e = self.graph[now][i];
                if e.cap > 0 && self.level[now] < self.level[e.to] {
                    let d = self.dfs(e.to, target, std::cmp::min(f, e.cap));
                    if d > 0 {
                        self.graph[now][i].cap -= d;
                        self.graph[e.to][e.rev].cap += d;
                        self.iter[now] = i;
                        return d;
                    }
                }
                i += 1;
            }
            self.iter[now] = i;
            0
        }
        pub fn flow(&mut self, start: usize, target: usize) -> i64 {
            let mut res = 0;
            loop {
                self.bfs(start);
                if self.level[target] < 0 { return res; }
                self.iter = vec![0; self.size];
                loop {
                    let f = self.dfs(start, target, INF);
                    if f <= 0 { break; }
                    res += f;
                }
            }
        }
    }
    pub struct FordFullkerson {
        size: usize,
        graph: Vec<Vec<Edge>>
    }
    #[allow(dead_code)]
    impl FordFullkerson {
        pub fn new(size: usize) -> Self {
            Self { size, graph: vec![vec![]; size] }
        }
        pub fn set_edge(&mut self, from: usize, to: usize, cap: i64) {
            let f_size = self.graph[from].len();
            let t_size = self.graph[to].len();
            self.graph[from].push(Edge::new(to, cap, t_size));
            self.graph[to].push(Edge::new(from, 0, f_size));
        }
        pub fn flow(&mut self, start: usize, goal: usize) -> i64 {
            let mut res = 0;
            loop {
                let mut used = vec![false; self.size];
                let f = self.dfs(start, INF, goal, &mut used);
                if f == 0 { return res; }
                res += f;
            }
        }
        fn dfs(&mut self, now: usize, f: i64, goal: usize, used: &mut Vec<bool>) -> i64 {
            if now == goal { return f; }
            used[now] = true;
            for i in 0..self.graph[now].len() {
                let Edge { to, cap, rev } = self.graph[now][i];
                if used[to] || cap == 0 { continue; }
                let res = self.dfs(to, std::cmp::min(f, cap), goal, used);
                if res == 0 { continue; }
                self.graph[to][rev].cap += res;
                self.graph[now][i].cap -= res;
                return res;
            }
            0
        }
    }
}
#[allow(unused_imports)]
use flow::FordFullkerson;
#[allow(unused_imports)]
use flow::Dinic;

fn main() {
    input! {n: usize, t: i64, a: [(i64, i64); n], b: [(i64, i64); n]};

    let mut ff = Dinic::new(n*2+2);
    let mut map = HashMap::new();
    for (i, &(x, y)) in b.iter().enumerate() {
        map.insert((x, y), i+n);
        ff.set_edge(i+n, n*2+1, 1);
    }
    let d = [(t, 0), (t, t), (0, t), (-t, t), (-t, 0), (-t, -t), (0, -t), (t, -t)];
    for (i, &(x, y)) in a.iter().enumerate() {
        for &(dx, dy) in &d {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(&to) = map.get(&(nx, ny)) {
                ff.set_edge(i, to, 1);
            }
        }
        ff.set_edge(n*2, i, 1);
    }

    let f = ff.flow(n*2, n*2+1);
    if f != n as i64 {
        println!("No");
        std::process::exit(0);
    }

    let t = ff.get_graph();
    let mut res = vec![0; n];
    for i in n..n*2 {
        for &flow::Edge { to, cap, rev: _ } in &t[i] {
            if i > to && cap > 0 {
                let (sx, sy) = a[to];
                let (tx, ty) = b[i-n];
                for j in 0..8 {
                    if d[j] == (tx-sx, ty-sy) {
                        res[to] = j+1;
                    }
                }
            }
        }
    }

    println!("Yes");
    for (i, v) in res.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v);
    }
    println!("");
}