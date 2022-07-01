use proconio::input;

struct SegmentTree<T>
where
    T: Clone + Copy + Sized,
{
    sz: usize,
    def_val: T,
    update_func: fn(T, T) -> T,
    tree: Vec<T>
}
impl<T> SegmentTree<T>
where
    T: Clone + Copy + Sized,
{
    fn new(size: usize, def_val: T, update_func: fn(T, T) -> T) -> Self {
        let mut sz = 1;
        while sz < size {
            sz <<= 1;
        }
        let tree = vec![def_val; sz * 2];
        Self { sz, def_val, update_func, tree }
    }
    fn update(&mut self, mut index: usize, val: T) {
        index += self.sz;
        self.tree[index] = val;
        let update_func = &self.update_func;
        while index >> 1 > 0 {
            index >>= 1;
            self.tree[index] = update_func(self.tree[index*2], self.tree[index*2+1]);
        }
    }
    fn get_sub(&self, left: usize, right: usize, now: usize, a: usize, b: usize) -> T {
        if right <= a || b <= left {
            return self.def_val;
        }
        if left <= a && b <= right {
            return self.tree[now];
        }
        let update_func = &self.update_func;
        let mut res = self.def_val;
        res = update_func(res, self.get_sub(left, right, now*2  , a, (a+b) / 2));
        res = update_func(res, self.get_sub(left, right, now*2+1, (a+b) / 2, b));
        res
    }
    fn get(&self, left: usize, right: usize) -> T {
        self.get_sub(left, right, 1, 0, self.sz)
    }
}

fn main() {
    input! {n: usize, q: usize, a: [i64; n], p: [(usize, usize, i64); q]};

    // let mut a = a;
    let mut b = a.iter().enumerate().skip(1).map(|(i, x)| x - a[i-1]).collect::<Vec<i64>>();
    let mut st = SegmentTree::new(n-1, 0i64, |x, y| x + y);
    for (i, v) in b.iter().enumerate() {
        st.update(i, v.abs());
    }

    for(l, r, v) in p {
        let l = l - 1;
        let r = r - 1;
        if l > 0 {
            b[l-1] += v;
            st.update(l-1, b[l-1].abs());
        }
        if r < n-1 {
            b[r] -= v;
            st.update(r, b[r].abs())
        }

        println!("{}", st.get(0, n-1));
    }
}