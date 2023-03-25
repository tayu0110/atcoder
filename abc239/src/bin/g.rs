use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], mut c: [i64; n]}
    c[0] = std::i64::MAX;
    c[n-1] = std::i64::MAX;

    // in: i, out: i+n
    let mut ff = flow::Dinic::new(n*2);
    for &(a, b) in &p {
        ff.set_edge(a-1+n, b-1, std::i64::MAX);
        ff.set_edge(b-1+n, a-1, std::i64::MAX);
    }
    for i in 0..n {
        ff.set_edge(i, i+n, c[i]);
    }

    let res_c = ff.flow(0, n-1);

    let graph = ff.graph;
    let mut reachable = vec![false; n*2];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back(0);
    while let Some(now) = nt.pop_front() {
        if reachable[now] {
            continue;
        }
        reachable[now] = true;

        for &flow::Edge { to, rev: _, cap, is_rev: _ } in &graph[now] {
            if cap <= 0 {
                continue;
            }
            nt.push_back(to);
        }
    }

    let mut res = vec![];
    for i in 0..n {
        if !reachable[i] {
            continue;
        }
        for &flow::Edge { to, rev: _, cap: _, is_rev: _ } in &graph[i] {
            if to == i + n && !reachable[to] {
                res.push(i+1);
            }
        }
    }

    println!("{}", res_c);
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
}

mod flow {
    pub trait Numeric: num::Num + std::cmp::PartialOrd + std::ops::AddAssign + std::ops::SubAssign {
        fn inf() -> Self;
        fn max(&self, rhs: Self) -> Self;
        fn min(&self, rhs: Self) -> Self;
    }
    impl Numeric for i64 {
        fn inf() -> Self { 0x7fffffffffffffffi64 }
        fn max(&self, rhs: Self) -> Self { std::cmp::max(*self, rhs) }
        fn min(&self, rhs: Self) -> Self { std::cmp::min(*self, rhs) }
    }
    impl Numeric for f64 {
        fn inf() -> Self { 1e300 }
        fn max(&self, rhs: Self) -> Self { if *self < rhs { rhs } else { *self } }
        fn min(&self, rhs: Self) -> Self { if *self < rhs { *self } else { rhs } }
    }
    #[derive(Clone, Copy)]
    pub struct Edge<Cap>
    where Cap: Clone + Copy + Numeric {
        pub to: usize,
        pub cap: Cap,
        pub rev: usize,
        pub is_rev: bool
    }
    impl<Cap> Edge<Cap>
    where Cap: Clone + Copy + Numeric {
        fn new(to: usize, cap: Cap, rev: usize, is_rev: bool) -> Self { Self { to, cap, rev, is_rev } }
    }
    pub struct Dinic<Cap>
    where Cap: Clone + Copy + Numeric {
        size: usize,
        level: Vec<i32>,
        iter: Vec<usize>,
        pub graph: Vec<Vec<Edge<Cap>>>
    }
    #[allow(dead_code)]
    impl<Cap> Dinic<Cap>
    where Cap: Clone + Copy + Numeric {
        pub fn new(size: usize) -> Self { Self { size, level: vec![0; size], iter: vec![0; size], graph: vec![vec![]; size] } }
        pub fn set_edge(&mut self, from: usize, to: usize, cap: Cap) {
            let (rev_from, rev_to) = (self.graph[to].len(), self.graph[from].len());
            self.graph[from].push(Edge::new(to, cap, rev_from, false));
            self.graph[to].push(Edge::new(from, Cap::zero(), rev_to, true));
        }
        fn bfs(&mut self, start: usize) {
            self.level = vec![-1; self.size];
            let mut nt: std::collections::VecDeque<(usize, i32)> = std::collections::VecDeque::new();
            nt.push_back((start, 0i32));
            while let Some((now, nd)) = nt.pop_front() {
                if self.level[now] >= 0 { continue; }
                self.level[now] = nd;
                self.graph[now].iter().for_each(|e| if e.cap > Cap::zero() && self.level[e.to] < 0 { nt.push_back((e.to, nd+1)) });
            }
        }
        fn dfs(&mut self, now: usize, target: usize, f: Cap) -> Cap {
            if now == target { return f; }
            let mut i = self.iter[now];
            while i < self.graph[now].len() {
                let e = self.graph[now][i];
                if e.cap > Cap::zero() && self.level[now] < self.level[e.to] {
                    let nf = if f < e.cap { f } else { e.cap };
                    let d = self.dfs(e.to, target, nf);
                    if d > Cap::zero() {
                        self.graph[now][i].cap -= d;
                        self.graph[e.to][e.rev].cap += d;
                        self.iter[now] = i;
                        return d;
                    }
                }
                i += 1;
            }
            self.iter[now] = i;
            Cap::zero()
        }
        pub fn flow(&mut self, start: usize, target: usize) -> Cap {
            let mut res = Cap::zero();
            loop {
                self.bfs(start);
                if self.level[target] < 0 { return res; }
                self.iter = vec![0; self.size];
                loop {
                    let f = self.dfs(start, target, Cap::inf());
                    if f <= Cap::zero() { break; }
                    res += f;
                }
            }
        }
    }
}
