use proconio::input;

fn mod_pow(a: i64, n: i64, p: i64) -> i64 {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let mut res = mod_pow(a, n / 2, p);
        res = res * res % p;
        if n % 2 == 1 {
            res = res * a % p;
        }
        res
    }
}

fn main() {
    input! {n: usize, m: i64, a: [i64; n]}

    let mut edges = vec![];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in a.iter().enumerate().skip(i + 1) {
            let xy = mod_pow(x, y, m);
            let yx = mod_pow(y, x, m);
            edges.push((-((xy + yx) % m), i, j));
        }
    }

    let mut res = 0;
    edges.sort();
    let mut uf = unionfind::UnionFind::new(n);
    for (w, i, j) in edges {
        if !uf.is_same(i, j) {
            res += -w;
            uf.merge(i, j);
        }
    }

    println!("{}", res);
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
