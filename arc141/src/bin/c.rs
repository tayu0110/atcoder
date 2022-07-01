macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
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
    input!(n: usize, p: [usize; n*2], q: [usize; n*2]);

    let p = p.iter().map(|x| x - 1).collect::<Vec<usize>>();
    let q = q.iter().map(|x| x - 1).collect::<Vec<usize>>();

    let mut pst = SegmentTree::new(n*2, 0, |x, y| x + y);
    let mut qst = SegmentTree::new(n*2, 0, |x, y| x + y);
    let mut pt = vec![0; n*2];

    for w in &p {
        let sum = pst.get(*w, n*2);
        pt[*w as usize] = sum;
        pst.update(*w, 1);
    }
    for w in &q {
        let sum = qst.get(*w, n*2);
        if sum % 2 != pt[*w as usize] % 2 {
            println!("-1");
            std::process::exit(0);
        }
        qst.update(*w, 1);
    }

    
}
