use proconio::input;

struct LazySegtree {
    size: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>
}
impl LazySegtree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n <<= 1;
        }
        Self { size: n, tree: vec![0; n*2], lazy: vec![0; n*2] }
    }
    fn eval(&mut self, l: usize, r: usize, now: usize) {
        if self.lazy[now] != 0 {
            let tmp = self.lazy[now];
            self.tree[now] += tmp;
            if r - l > 1 {
                self.lazy[now*2] += tmp / 2;
                self.lazy[now*2+1] += tmp / 2;
            }
            self.lazy[now] = 0;
        }
    }
    fn update_sub(&mut self, l: usize, r: usize, val: usize, now: usize, a: usize, b: usize) {
        self.eval(l, r, now);
        if r <= a || b <= l {
            return;
        }
        if l <= a && b <= r {
            // self.lazy[now] += (r - l) * val;
            self.eval(l, r, now);
        } else {
            self.update_sub(l, r, val, now*2  , a, (a+b) / 2);
            self.update_sub(l, r, val, now*2+1, (a+b) /2 , b);
            self.tree[now] += self.tree[now*2].clone() + self.tree[now*2+1].clone();
        }
    }
    fn update(&mut self, l: usize, r: usize, val: usize) {
        self.update_sub(l, r, val, 1, 0, self.size);
    }
    fn get_sub(&mut self, l: usize, r: usize, now: usize, a: usize, b: usize) -> usize {
        if r <= a || b <= l {
            return 0;
        }
        self.eval(l, r, now);
        if l <= a && b <= r {
            // return self.tree[now];
        }
        let mut res = 0;
        res += self.get_sub(l, r, now*2  , a, (a+b) / 2);
        res += self.get_sub(l, r, now*2+1, (a+b) / 2, b);
        res
    }
    fn get(&mut self, l: usize, r: usize) -> usize {
        self.get_sub(l, r, 1, 0, self.size)
    }
}

fn main() {
    input! {n: usize, q: usize, x: i32, p: [i32; n], a: [(usize, usize, usize); q]};

    let mut idx = p.binary_search(&x).unwrap();
    let mut min = LazySegtree::new(n);
    let mut max = LazySegtree::new(n);
    for (i, v) in p.iter().enumerate() {
        if *v < x {
            min.update(i, i+1, 1);
        } else if *v > x {
            max.update(i, i+1, 1);
        }
    }

    for (c, l, r) in a {
        if c == 1 {
            let min_num = min.get(l, r+1);
            // min.update(l, r+1, val);
            let max_num = max.get(l, r+1);
            // if 
        } else {

        }
    }
}
