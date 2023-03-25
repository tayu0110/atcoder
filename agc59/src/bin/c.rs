use proconio::input;

fn main() {
    const MOD: usize = 1000_000_007;
    input! {n: usize, p: [(usize, usize); n * (n - 1) / 2]}

    let mut uf = unionfind::UnionFind::new(n * n);
    let mut memo = vec![vec![false; n]; n];
    let get_index = |from: usize, to: usize| from * n + to;
    for (x, y) in p.into_iter().map(|(x, y)| (x - 1, y - 1)) {
        memo[x][y] = true;
        memo[y][x] = true;
        for j in (0..n).filter(|&j| j != x && j != y) {
            // already exist x -> j and not exist j -> y
            if memo[j][x] && !memo[j][y] {
                uf.merge(get_index(j, x), get_index(y, x));
                uf.merge(get_index(x, y), get_index(x, j));
            }
            // already exist y -> j and not exist j -> x
            if memo[y][j] && !memo[x][j] {
                uf.merge(get_index(y, x), get_index(y, j));
                uf.merge(get_index(x, y), get_index(j, y));
            }
        }
    }

    let mut res = 1;
    let mut k = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (d, rd) = (get_index(i, j), get_index(j, i));
            if uf.is_same(d, rd) {
                res = 0;
            }

            if uf.root(d) == d {
                k += 1;
            }
            if uf.root(rd) == rd {
                k += 1;
            }
        }
    }

    if res == 0 {
        println!("0");
        return;
    }

    eprintln!("k: {}", k);
    assert!(k % 2 == 0);

    for _ in 0..k / 2 {
        res *= 2;
        res %= MOD;
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
