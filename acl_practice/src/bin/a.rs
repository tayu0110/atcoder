use proconio::input;

#[proconio::fastout]
fn main() {
    input! {n: usize, q: usize, p: [(usize, usize, usize); q]};

    let mut uf = unionfind::UnionFind::new(n);
    for (t, u, v) in p {
        if t == 0 {
            uf.merge(u, v);
        } else if uf.is_same(u, v) {
            println!("1");
        } else {
            println!("0");
        }
    }
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>
    }
    impl UnionFind {
        #[inline]
        pub fn new(size: usize) -> Self {
            UnionFind { tree: vec![-1; size] }
        }
        #[inline]
        pub fn root(&mut self, index: usize) -> usize {
            if self.tree[index] < 0 {
                index
            } else {
                self.tree[index] = self.root(self.tree[index] as usize) as i32;
                self.tree[index] as usize
            }
        }
        #[inline]
        pub fn size(&mut self, index: usize) -> usize {
            let root = self.root(index);
            -self.tree[root] as usize
        }
        #[inline]
        pub fn is_same(&mut self, left: usize, right: usize) -> bool {
            self.root(left) == self.root(right)
        }
        #[inline]
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
