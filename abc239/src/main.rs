use rand::{self, Rng};
use std::io::Write;

fn main() {
    const N: usize = 100;
    let mut rng = rand::thread_rng();
    let mut file = std::fs::File::create("input.txt").unwrap();

    writeln!(file, "{}", N).unwrap();

    let mut uf = unionfind::UnionFind::new(N);
    let mut set = std::collections::HashSet::new();
    for _ in 0..N-1 {
        let (mut a, mut b): (usize, usize) = (rng.gen_range(1, N+1), rng.gen_range(1, N+1));
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        while (a == 1 && b == N) || (a == b) || set.contains(&(a, b)) {
            a = rng.gen_range(1, N+1);
            b = rng.gen_range(1, N+1);
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
        }
        set.insert((a, b));
        uf.merge(a-1, b-1);
    }

    for i in 1..N-1 {
        if uf.root(i) != uf.root(0) {
            set.insert((1, i+1));
            uf.merge(0, i);
        }
        if uf.root(i) != uf.root(N-1) {
            set.insert((i+1, N));
            uf.merge(i, N-1);
        }
    }

    writeln!(file, "{}", set.len()).unwrap();
    for (a, b) in set {
        writeln!(file, "{} {}", a, b).unwrap();
    }

    const C_MIN: usize = 800_000_000;
    const C_MAX: usize = 1_000_000_000;
    write!(file, "0").unwrap();
    for _ in 0..N-2 {
        let c: usize = rng.gen_range(C_MIN, C_MAX+1);
        write!(file, " {}", c).unwrap();
    }
    writeln!(file, " 0").unwrap();
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind { tree: vec![-1; size] }
        }
        pub fn root(&mut self, index: usize) -> usize {
            if self.tree[index] < 0 {
                index
            } else {
                self.tree[index] = self.root(self.tree[index] as usize) as i32;
                self.tree[index] as usize
            }
        }
        pub fn size(&mut self, index: usize) -> usize {
            let root = self.root(index);
            -self.tree[root] as usize
        }
        pub fn is_same(&mut self, left: usize, right: usize) -> bool {
            self.root(left) == self.root(right)
        }
        pub fn merge(&mut self, left: usize, right: usize) -> bool {
            let (mut rl, mut rr) = (self.root(left), self.root(right));
            if rl == rr {
                return false;
            }
            if self.tree[rl] > self.tree[rr] {
                std::mem::swap(&mut rl, &mut rr);
            }
            self.tree[rl] += self.tree[rr];
            self.tree[rr] = rl as i32;
            true
        }
    }
}
