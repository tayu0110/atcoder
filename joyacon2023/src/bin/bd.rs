use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut v = vec![];
    for &(a, b) in &p {
        v.push(a);
        v.push(b);
    }

    v.sort();
    v.dedup();

    if v[0] != 1 {
        println!("1");
        return;
    }

    let mut map = std::collections::HashMap::new();
    for (i, v) in v.iter().enumerate() {
        map.insert(v, i);
    }

    let mut uf = unionfind::UnionFind::new(v.len());
    for (a, b) in p {
        let (a, b) = (*map.get(&a).unwrap(), *map.get(&b).unwrap());

        uf.merge(a, b);
    }

    let mut max = 0;
    for i in 0..v.len() {
        if !uf.is_same(i, 0) {
            continue;
        }

        max = std::cmp::max(v[i], max);
    }

    println!("{}", max);
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
