#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

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
        pub rev: usize
    }
    impl<Cap> Edge<Cap>
    where Cap: Clone + Copy + Numeric {
        fn new(to: usize, cap: Cap, rev: usize) -> Self { Self { to, cap, rev } }
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
            self.graph[from].push(Edge::new(to, cap, rev_from));
            self.graph[to].push(Edge::new(from, Cap::zero(), rev_to));
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
#[allow(unused_imports)]
use flow::Dinic;

fn main() {
    input! {n: usize, m: usize, mut s: [Chars; n]};

    let (start, target) = (0, n * m + 1);
    let mut ff = Dinic::new(n*m+2);
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let mut ck = std::collections::HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '#' {
                continue;
            }
            if (i + j) % 2 == 1 {
                ff.set_edge(start, i*m+j+1, 1);
                ck.insert((start, i*m+j+1));
            } else {
                ff.set_edge(i*m+j+1, target, 1);
                ck.insert((i*m+j+1, target));
                for k in 0..4 {
                    let nx = j as i32 + dx[k];
                    let ny = i as i32 + dy[k];
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    if s[ny as usize][nx as usize] == '#' {
                        continue;
                    }
                    ck.insert((ny as usize * m + nx as usize + 1, i * m + j + 1));
                    ff.set_edge(ny as usize * m + nx as usize + 1, i * m + j + 1, 1);
                }
            }
        }
    }

    println!("{}", ff.flow(start, target));

    for i in 0..n*m+2 {
        for flow::Edge { to, rev: _, cap} in &ff.graph[i] {
            // eprintln!("to: {}, rev: {}, cap: {}", to, rev, cap);
            if !ck.contains(&(i, *to)) {
                continue;
            }
            if cap == &0 && i != 0 && *to != n*m+1 {
                let (mut a, mut b) = (i, *to);
                // eprintln!("i: {}, to: {}", i, to);
                a -= 1;
                b -= 1;
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                if a / m != b / m {
                    eprintln!("a: {}, b: {}, m: {}", a, b, m);
                    s[a / m][a % m] = 'v';
                    s[b / m][b % m] = '^';
                } else {
                    s[a / m][a % m] = '>';
                    s[b / m][b % m] = '<';
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
