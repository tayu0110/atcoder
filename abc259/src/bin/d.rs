#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

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
            let tmp = rl;
            rl = rr;
            rr = tmp;
        }
        self.tree[rl] += self.tree[rr];
        self.tree[rr] = rl as i32;
        return true;
    }
}

fn main() {
	input! {n: usize, sx: i64, sy: i64, tx: i64, ty: i64, p: [(i64, i64, i64); n]};

    let mut sbuf = vec![];
    let mut tbuf = vec![];

    for (i, (x, y, r)) in p.iter().enumerate() {
        let (x, y, r) = (*x, *y, *r);
        if (x - sx) * (x - sx) + (y - sy) * (y - sy) == r * r {
            sbuf.push(i);
        }
        if (x - tx) * (x - tx) + (y - ty) * (y - ty) == r * r {
            tbuf.push(i);
        }
    }

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i+1..n {
            let (sx, sy, sr) = p[i];
            let (tx, ty, tr) = p[j];
            let dr = (sx - tx) * (sx - tx) + (sy - ty) * (sy - ty);
            if dr > (sr + tr) * (sr + tr) {
                continue;
            }
            if dr < sr * sr + tr * tr - 2 * sr * tr {
                continue;
            }
            if (sx - tx) * (sx - tx) + (sy - ty) * (sy - ty) <= (sr + tr) * (sr + tr) {
                uf.merge(i, j);
            }
        }
    }

    for v in sbuf {
        for w in &tbuf {
            let w = *w;
            if uf.is_same(v, w) {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }

    println!("No");
}
