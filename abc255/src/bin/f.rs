use std::collections::VecDeque;

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

const INF: usize = 111222333444555666;

fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]};

    if p[0] != 1 {
        println!("-1");
        std::process::exit(0);
    }

    let p = p.iter().map(|x| *x - 1).collect::<Vec<usize>>();
    let q = q.iter().map(|x| *x - 1).collect::<Vec<usize>>();

    let mut np = p.iter().enumerate().map(|(l, r)| (*r, l)).collect::<Vec<(usize, usize)>>();
    np.sort();
    let nq = q.iter().map(|x| np[*x].1).collect::<Vec<usize>>();
    np.sort_by(|l, r| l.1.cmp(&r.1));

    let mut st = SegmentTree::new(n, INF, |l, r| std::cmp::min(l, r));
    let mut indices = vec![0; n];
    for (i, v) in nq.iter().enumerate() {
        st.update(i, *v);
        indices[*v] = i;
    }

    let mut nt = VecDeque::new();
    let mut res = vec![(0, 0); n];
    nt.push_back((0, 0, n));
    while !nt.is_empty() {
        let (now, l, r) = nt.pop_front().unwrap();
        let mut childs = (0, 0);

        if l != indices[now] {
            let lmin = st.get(l, indices[now]);
            childs.0 = np[lmin].0 + 1;
            nt.push_back((lmin, l, indices[now]));
        }
        if r != indices[now] + 1 {
            let rmin = st.get(indices[now]+1, r);
            childs.1 = np[rmin].0 + 1;
            nt.push_back((rmin, indices[now]+1, r));
        }

        res[np[now].0] = childs;
    }

    let mut confirm = vec![];
    let mut nt = VecDeque::new();
    nt.push_back(0);
    while !nt.is_empty() {
        let now = nt.pop_back().unwrap();
        confirm.push(now);

        let (l, r) = res[now];
        if r != 0 {
            nt.push_back(r-1);
        }
        if l != 0 {
            nt.push_back(l-1);
        }
    }

    if confirm.len() != p.len() {
        println!("-1");
        std::process::exit(0);
    }

    for (l, r) in confirm.iter().zip(p.iter()) {
        if l != r {
            println!("-1");
            std::process::exit(0);
        }
    }

    for (l, r) in res {
        println!("{} {}", l, r);
    }
}