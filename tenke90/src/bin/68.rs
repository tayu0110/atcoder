use proconio::input;

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

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize, usize, i64); q]};

    let mut uf = UnionFind::new(n+1);
    let mut np = vec![];
    let mut list = vec![];

    for (t, x, y, v) in p {
        if t == 0 {
            uf.merge(x, y);
            list.push((x, y, v));
        } else if !uf.is_same(x, y) {
            np.push((0, 0, -1));
        } else {
            np.push((x, y, v));
        }
    }

    list.sort();
    let mut t = vec![0; n+1];
    for (x, y, v) in list {
        t[y] = v - t[x];
    }

    eprintln!("{:?}", t);

    for (x, y, v) in np {
        if v < 0 {
            println!("Ambiguous");
        } else if x % 2 == y % 2 {
            println!("{}", v - t[x] + t[y]);
        } else {
            println!("{}", t[y] - v + t[x]);
        }
    }
}