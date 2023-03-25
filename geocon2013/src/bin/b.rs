#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

#[allow(dead_code)]
mod mincost_flow {
    const MAX_LEVEL: usize = 0xffffffffffffffffusize;
    pub trait Numeric: num::Num + std::cmp::PartialOrd + std::ops::AddAssign + std::ops::SubAssign + std::ops::Neg {
        fn inf() -> Self;
        fn max(&self, rhs: Self) -> Self;
        fn min(&self, rhs: Self) -> Self;
    }
    impl Numeric for i64 {
        fn inf() -> Self { 0x3f3f3f3f3f3f3f3fi64 }
        fn max(&self, rhs: Self) -> Self { std::cmp::max(*self, rhs) }
        fn min(&self, rhs: Self) -> Self { std::cmp::min(*self, rhs) }
    }
    impl Numeric for f64 {
        fn inf() -> Self { 1e100 }
        fn max(&self, rhs: Self) -> Self { if *self < rhs { rhs } else { *self } }
        fn min(&self, rhs: Self) -> Self { if *self < rhs { *self } else { rhs } }
    }
    #[derive(Clone, Copy)]
    pub struct Edge<T>
    where T: Clone + Copy + Numeric {
        to: usize,
        rev: usize,
        cap: T,
        cost: T
    }
    pub struct MinCostFlow<T>
    where T: Clone + Copy + Numeric {
        size: usize,
        graph: Vec<Vec<Edge<T>>>,
        level: Vec<usize>
    }
    impl<T> MinCostFlow<T>
    where T: Clone + Copy + Numeric + std::ops::Neg<Output = T> + std::fmt::Display + std::fmt::Debug {
        pub fn new(size: usize) -> Self { Self { size, graph: vec![vec![]; size], level: vec![MAX_LEVEL; size] } }
        pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) {
            assert!(from < self.size && to < self.size);
            let (fi, ti) = (self.graph[from].len(), self.graph[to].len());
            self.graph[from].push(Edge { to, rev: ti, cap, cost });
            self.graph[to].push(Edge { to: from, rev: fi, cap: T::zero(), cost });
        }
        pub fn graph(&self) -> &Vec<Vec<Edge<T>>> { &self.graph }
        pub fn flow(&mut self, from: usize, to: usize) -> T {
            self.flow_with_cap(from, to, T::inf())
        }
        pub fn flow_with_cap(&mut self, from: usize, to: usize, mut cap: T) -> T {
            assert!(from < self.size && to < self.size);
            let mut res_cost = T::zero();
            while cap > T::zero() {
                let (rcap, rcost) = self.bellman_ford(from, to, cap);
                eprintln!("rcap: {}, rcost: {}", rcap, rcost);
                if rcap == T::inf() {
                    res_cost = T::inf();
                    break;
                }
                res_cost += rcap * rcost;
                cap -= rcap;
                eprintln!("cap: {}", cap);
            }
            res_cost
        }
        fn bellman_ford(&mut self, from: usize, to: usize, cap: T) -> (T, T) {
            let mut costs = vec![T::inf(); self.size];
            let mut before = vec![MAX_LEVEL; self.size];
            costs[from] = T::zero();
            for _ in 0..self.size-1 {
                for from in 0..self.size {
                    for Edge { to, rev: _, cap: ncap, cost } in &self.graph[from] {
                        if *ncap > cap {
                            continue;
                        }
                        if costs[from] + *cost < costs[*to] {
                            costs[*to] = costs[from] + *cost;
                            before[*to] = from;
                        }
                    }
                }
            }
            for from in 0..self.size {
                for Edge { to, rev: _, cap: ncap, cost } in &self.graph[from] {
                    if *ncap > cap {
                        continue;
                    }
                    if costs[from] + *cost < costs[*to] {
                        return (T::inf(), -T::inf());
                    }
                }
            }
            let mut route = vec![];
            let mut now = to;
            let mut res_cap = T::inf();
            loop {
                route.push(now);
                if before[now] == MAX_LEVEL {
                    return (T::inf(), T::inf());
                }
                for i in 0..self.graph[before[now]].len() {
                    if self.graph[before[now]][i].to == now {
                        if self.graph[before[now]][i].cap < res_cap {
                            res_cap = self.graph[before[now]][i].cap;
                        }
                    }
                }
                now = before[now];
                if now == from {
                    break;
                }
            }
            for to in route {
                let from = before[to];
                let mut rev = 0;
                for Edge { to: nto, rev: nrev, cap, cost: _ } in self.graph[from].iter_mut() {
                    if *nto == to {
                        *cap -= res_cap;
                        rev = *nrev;
                    }
                }
                self.graph[to][rev].cap += res_cap;
            }
            (res_cap, costs[to])
        }
    }
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]};

    let mut ff = mincost_flow::MinCostFlow::new(n*2+2);
    let v = n * 2 + 2;
    let (s, t) = (v - 2, v - 1);
    for (i, &(x, y)) in p.iter().enumerate() {
        for (j, &(tx, ty)) in p.iter().enumerate() {
            if i == j {
                let cost = (((x - tx) * (x - tx) + y * y) as f64).sqrt();
                ff.add_edge(i, n+j, 1.0, cost);
            } else {
                let cost = (((x + tx) * (x + tx) + (y - ty) * (y - ty)) as f64).sqrt();
                ff.add_edge(i, n+j, 1.0, cost);
            }
        }
    }
    for i in 0..n {
        ff.add_edge(s, i, 1.0, 0.0);
        ff.add_edge(n + i, t, 1.0, 0.0);
    }

    let res = ff.flow_with_cap(s, t, n as f64);
    println!("{}", res);
    // let graph = ff.graph();

}
