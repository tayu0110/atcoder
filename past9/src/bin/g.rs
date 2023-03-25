use proconio::input;

fn main() {
    input! {n: usize, q: usize}

    let mut set = std::collections::HashSet::new();

    for _ in 0..q {
        input! {t: usize, mut u: usize, mut v: usize}
        u -= 1;
        v -= 1;

        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        if t == 1 {
            if set.contains(&(u, v)) {
                set.remove(&(u, v));
            } else {
                set.insert((u, v));
            }
        } else {
            let mut uf = unionfind::UnionFind::new(n);
            for &(u, v) in &set {
                uf.merge(u, v);
            }

            if uf.is_same(u, v) {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>,
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind {
                tree: vec![-1; size],
            }
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
