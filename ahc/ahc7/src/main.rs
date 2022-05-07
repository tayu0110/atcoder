use std::io::Write;

struct UnionFind {
    par: Vec<i32>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { par: vec![-1; n] }
    }
    fn root(&mut self, idx: usize) -> usize {
        if self.par[idx] < 0 {
            return idx;
        }
        self.par[idx] = self.root(self.par[idx] as usize) as i32;
        self.par[idx] as usize
    }
    fn size(&mut self, idx: usize) -> i32 {
        let root = self.root(idx);
        -self.par[root]
    }
    fn merge(&mut self, lhs: usize, rhs: usize) -> bool {
        let mut parent = self.root(lhs);
        let mut child = self.root(rhs);
        if self.size(parent) < self.size(child) {
            std::mem::swap(&mut parent, &mut child);
        }
        self.par[parent] += self.par[child];
        self.par[child] = parent as i32;
        true
    }
    fn is_same(&mut self, lhs: usize, rhs: usize) -> bool {
        self.root(lhs) == self.root(rhs)
    }
}
fn main() {
    let n: usize = 400;
    let m: usize = 1995;
    let mut point = Vec::new();
    for _ in 0..n {
        let xy = {
            let mut xy = String::new();
            std::io::stdin().read_line(&mut xy).unwrap();
            xy.trim_end().to_owned()
        };
        let (x, y) = {
            let mut tmp = xy.split_whitespace();
            let x: usize = tmp.next().unwrap().parse().unwrap();
            let y: usize = tmp.next().unwrap().parse().unwrap();
            (x, y)
        };
        point.push((x, y));
    }
    let mut graph = Vec::new();
    for _ in 0..m {
        let uv = {
            let mut uv = String::new();
            std::io::stdin().read_line(&mut uv).unwrap();
            uv.trim_end().to_owned()
        };
        let (u, v) = {
            let mut tmp = uv.split_whitespace();
            let u: usize = tmp.next().unwrap().parse().unwrap();
            let v: usize = tmp.next().unwrap().parse().unwrap();
            (u, v)
        };
        graph.push((u, v));
    }
    let mut uf = UnionFind::new(n);
    for i in 0..m {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };
        let _: i32 = s.parse().unwrap();
        let (u, v) = graph[i];
        if !uf.is_same(u, v) {
            println!("{}", 1);
            std::io::stdout().flush().unwrap();
            uf.merge(u, v);
        } else {
            println!("{}", 0);
            std::io::stdout().flush().unwrap();
        }
    }
}
