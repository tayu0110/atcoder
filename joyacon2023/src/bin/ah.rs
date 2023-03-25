use proconio::input;

fn main() {
    input! {n: usize, m: usize}

    let mut check = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {k: usize, x: [usize; k]}

        for j in 0..k {
            for l in j + 1..k {
                let (s, t) = (x[j] - 1, x[l] - 1);
                check[s][t] = true;
                check[t][s] = true;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if !check[i][j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
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
