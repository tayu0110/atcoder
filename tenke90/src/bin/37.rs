use proconio::input;

struct DualSegtree {
    size: usize,
    tree: Vec<i64>
}
impl DualSegtree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        Self { size: n, tree: vec![-1; n*2] }
    }
    fn update_sub(&mut self, l: usize, r: usize, val: i64, now: usize, a: usize, b: usize) {
        if r <= a || b <= l {
            return;
        }
        if l <= a && b <= r {
            self.tree[now] = std::cmp::max(self.tree[now], val);
            return;
        }
        self.update_sub(l, r, val, now*2  , a, (a+b) / 2);
        self.update_sub(l, r, val, now*2+1, (a+b) / 2, b);
    }
    fn update(&mut self, l: usize, r: usize, val: i64) {
        self.update_sub(l, r, val, 1, 0, self.size);
    }
    fn get(&self, idx: usize) -> i64 {
        let mut idx = idx + self.size;
        let mut res = -1;
        while idx > 0 {
            res = std::cmp::max(res, self.tree[idx]);
            idx >>= 1;
        }
        res
    }
}

fn main() {
    input! {w: usize, n: usize, p: [(usize, usize, i64); n]};

    let mut st = DualSegtree::new(w+1);
    st.update(0, 1, 0);

    for (l, r, v) in p {
        let r = r + 1;
        for i in (0..w+1).rev() {
            if i+l > w {
                continue;
            }
            let now = st.get(i);
            if now < 0 {
                continue;
            }
            let l = i + l;
            let r = if i+r <= w { i+r } else { w+1 };
            st.update(l, r, now + v);
        }
    }

    let res = st.get(w);
    println!("{}", res);
}