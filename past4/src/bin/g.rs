use proconio::input;
use proconio::marker::Chars;

fn check(n: usize, m: usize, s: &Vec<Vec<char>>) -> bool {
    let mut uf = unionfind::UnionFind::new(n * m);
    let get_index = |i: usize, j: usize| i * m + j;

    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '#' {
                continue;
            }
            if i + 1 < n && s[i + 1][j] == '.' {
                uf.merge(get_index(i, j), get_index(i + 1, j));
            }
            if j + 1 < m && s[i][j + 1] == '.' {
                uf.merge(get_index(i, j), get_index(i, j + 1));
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '#' {
                continue;
            }
            set.insert(uf.root(get_index(i, j)));
        }
    }

    set.len() == 1
}

fn main() {
    input! {n: usize, m: usize, mut s: [Chars; n]}

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '.' {
                continue;
            }

            s[i][j] = '.';
            if check(n, m, &s) {
                res += 1;
            }

            s[i][j] = '#';
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
