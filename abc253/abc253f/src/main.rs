use proconio::input;

struct PermanentDualSegTree
{
    size: usize,
    tree: Vec<Vec<(usize, usize)>>
}
impl PermanentDualSegTree
{
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        Self { size: n, tree: vec![vec![]; n*2] }
    }
    fn update_sub(&mut self, l: usize, r: usize, val: (usize, usize), now: usize, a: usize, b: usize) {
        if b <= l || r <= a {
            return;
        }
        if l <= a && b <= r {
            if self.tree[now].is_empty() {
                self.tree[now].push(val);
            } else {
                let (_, sum) = &self.tree[now].last().unwrap().clone();
                self.tree[now].push((val.0, val.1 + *sum));
            }
        } else {
            self.update_sub(l, r, val, now*2,   a, (a+b) / 2);
            self.update_sub(l, r, val, now*2+1, (a+b) / 2, b);
        }
    }
    fn update(&mut self, l: usize, r: usize, val: (usize, usize)) {
        self.update_sub(l, r, val, 1, 0, self.size);
    }
    fn get(&self, idx: usize, gen: usize) -> usize {
        let mut idx = idx + self.size;
        let mut res = 0;
        while idx > 0 {
            if self.tree[idx].len() > 0 {
                let mut l = -1;
                let mut r = self.tree[idx].len() as i32;
                while r - l > 1 {
                    let m = (r + l) / 2;
                    if self.tree[idx][m as usize].0 <= gen {
                        l = m;
                    } else {
                        r = m;
                    }
                }
                res += self.tree[idx].last().unwrap().1;
                if l >= 0 {
                    res -= self.tree[idx][l as usize].1;
                }
            }
            idx >>= 1;
        }
        res
    }
}

fn main() {
    input!(n: usize, m: usize, q: usize);

    let mut row = vec![(0, 0); n];
    let mut st = PermanentDualSegTree::new(m);
    let mut res = vec![];

    for nq in 1..q+1 {
        input! {t: usize};

        if t == 1 {
            input! {l: usize, r: usize, x: usize};
            st.update(l-1, r, (nq, x));
        } else if t == 2 {
            input! {i: usize, x: usize};
            row[i-1] = (nq, x);
        } else {
            input! {i: usize, j: usize};
            let (gen, x) = row[i-1];
            res.push(x + st.get(j-1, gen));
        }

        // eprintln!("{:?}", st.tree);
    }

    for v in res {
        println!("{}", v);
    }

    // let mut row = vec![(-1, 0usize); n];
    // let mut col = LazySegtree::new(m);

    // let mut res = vec![];
    // for nq in 0..q {
    //     input!(t: usize);

    //     if t == 1 {
    //         input!(l: usize, r: usize, x: usize);
    //         col.update(l-1, r, (nq as i32, x));
    //     } else if t == 2 {
    //         input!(i: usize, x: usize);
    //         row[i-1] = (nq as i32, x);
    //     } else {
    //         input!(i: usize, j: usize);
    //         let (pq, x) = row[i-1];
    //         res.push(x + col.get(j-1, j, pq));
    //     }
    // }

    // for v in res {
    //     println!("{}", v);
    // }
}
