#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut uf = UnionFind::new(n);
    let mut t = vec![vec![]; n];
    for (x, y, z) in p {
        t[x-1].push((y-1, z));
        t[y-1].push((x-1, z));
        uf.merge(x-1, y-1);
    }

    let mut diceded = vec![false; n];
    let mut res = 0;
    for now in 0..n {
        if diceded[uf.root(now)] {
            continue;
        }
        let mut cnt = 0;
        for j in 0..2 {
            let mut ck = std::collections::HashMap::new();
            *ck.entry(now).or_default() = j;
            if check(now, &mut ck, &t) {
                cnt += 1;
            }
        }
        if cnt != 1 {
            res += 1;
        }

        diceded[uf.root(now)] = true;
    }

    println!("{}", res);
}

fn check(now: usize, ck: &mut std::collections::HashMap<usize, usize>, t: &Vec<Vec<(usize, usize)>>) -> bool {
    for (to, z) in &t[now] {
        let nt = (ck.get(&now).unwrap() + *z) % 2;
        if let Some(v) = ck.get(to) {
            if v != &nt {
                return false;
            }
            return true;
        }
        *ck.entry(*to).or_default() = nt;
        let f = check(*to, ck, t);
        if !f {
            return f;
        }
    }

    true
}

struct UnionFind {
    tree: Vec<i32>
}
impl UnionFind {
    fn new(size: usize) -> Self {
        let mut tree = vec![];
        for _ in 0..size {
            tree.push(-1);
        }
        UnionFind { tree }
    }
    #[allow(dead_code)]
    fn root(&self, index: usize) -> usize {
        if self.tree[index] < 0 {
            index
        } else {
            self.root(self.tree[index] as usize)
        }
    }
    #[allow(dead_code)]
    fn size(&self, index: usize) -> usize {
        -self.tree[self.root(index)] as usize
    }
    #[allow(dead_code)]
    fn is_same(&self, left: usize, right: usize) -> bool {
        self.root(left) == self.root(right)
    }
    #[allow(dead_code)]
    fn merge(&mut self, left: usize, right: usize) -> bool {
        let mut rl = self.root(left);
        let mut rr = self.root(right);
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