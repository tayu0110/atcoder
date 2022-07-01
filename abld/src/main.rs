macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inter!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        #[allow(unused_mut)]
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_ascii_whitespace();
        input_inter!{iter, $($r)*}
    };
}
macro_rules! input_inter {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inter!{$iter $($r)*}
    };
}
macro_rules! read_value {
    ($iter:expr, ( $($t:tt), *)) => {
        ( $(read_value!($iter, $t)), *)
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

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
    input!(n: usize, k: usize, a: [usize; n]);
    const MX: usize = 300010;

    let mut st = SegmentTree::new(MX, 0, |lhs, rhs| std::cmp::max(lhs, rhs));
    let mut dp = vec![1; n];
    let mut res = 0;
    for (i, v) in a.iter().enumerate() {
        let l = if *v >= k { v - k } else { 0 };
        let r = std::cmp::min(MX, v + k + 1);
        dp[i] = std::cmp::max(dp[i], st.get(l, r) + 1);
        st.update(*v, dp[i]);
        res = std::cmp::max(res, dp[i]);
    }
    
    println!("{}", res);
}
