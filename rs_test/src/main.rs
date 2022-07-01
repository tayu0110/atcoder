// use proconio::input;

// struct Input<T>
// where T: std::io::BufRead
// {
//     reader: T,
//     buf: std::collections::VecDeque<String>
// }
// impl<T> Input<T>
// where T: std::io::BufRead
// {
//     fn new(mut reader: T) -> Self {
//         let mut s = String::new();
//         if let Err(_) = reader.read_line(&mut s) {
//             panic!("Nothing to read.");
//         }
//         let buf = std::collections::VecDeque::from_iter(s.split_ascii_whitespace().into_iter().map(|s| s.to_string()));
//         Self { reader, buf }
//     }
//     fn update_buf(&mut self) {
//         let mut s = String::new();
//         if let Err(_) = self.reader.read_line(&mut s) {
//             panic!("Nothing to read.");
//         }
//         self.buf = std::collections::VecDeque::from_iter(s.split_ascii_whitespace().into_iter().map(|s| s.to_string()));
//     }
//     fn gets(&mut self) -> String {
//         if self.buf.is_empty() {
//             self.update_buf();
//         }
//         self.buf.pop_front().unwrap()
//     }
// }

mod segtree {
    pub struct LazySegtree<S, F> {
        n: usize,
        size: usize,
        log: usize,
        pub d: Vec<S>,
        pub lz: Vec<F>,
        op: fn(S, S) -> S,
        e: fn() -> S,
        id: fn() -> F,
        mapping: fn(F, S) -> S,
        composition: fn(F, F) -> F
    }
    #[allow(dead_code)]
    impl<S, F> LazySegtree<S, F>
    where
        S: Copy + Clone + Sized,
        F: Copy + Clone + Sized
    {
        pub fn new(size: usize, op: fn(S, S) -> S, e: fn() -> S, id: fn() -> F, mapping: fn(F, S) -> S, composition: fn(F, F) -> F) -> Self {
            LazySegtree::from_vec(&vec![e(); size], op, e, id, mapping, composition)
        }
        pub fn from_vec(v: &Vec<S>, op: fn(S, S) -> S, e: fn() -> S, id: fn() -> F, mapping: fn(F, S) -> S, composition: fn(F, F) -> F) -> Self {
            let n = v.len();
            let log = {
                let mut size = 1;
                let mut log = 0;
                while size < n {
                    size <<= 1;
                    log += 1;
                }
                log
            };
            let size = 1 << log;
            let mut d = vec![e(); size * 2];
            let lz = vec![id(); size * 2];
            for (i, w) in v.iter().enumerate() {
                d[size + i] = *w;
            }
            for i in (0..size).skip(1).rev() {
                d[i] = op(d[i*2], d[i*2 + 1]);
            }
            Self { n, size, log, d, lz, op, e, id, mapping, composition }
        }
        pub fn set(&mut self, idx: usize, val: S) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log+1).skip(1).rev() {
                self.push(idx >> i);
            }
            self.d[idx] = val;
            for i in 1..=self.log {
                self.update(idx >> i);
            }
        }
        pub fn get(&mut self, idx: usize) -> S {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log+1).skip(1).rev() {
                self.push(idx >> i);
            }
            self.d[idx]
        }
        pub fn prod(&mut self, l: usize, r: usize) -> S {
            assert!(l <= r && r <= self.n);
            let e = self.e;
            if l == r {
                return e();
            }
            let mut l = l + self.size;
            let mut r = r + self.size;
            for i in (0..self.log+1).skip(1).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }
            let mut sml = e();
            let mut smr = e();
            let op = self.op;
            while l < r {
                if (l & 1) != 0 {
                    sml = op(sml, self.d[l]);
                    l += 1;
                }
                if (r & 1) != 0 {
                    r -= 1;
                    smr = op(self.d[r], smr);
                }
                l >>= 1;
                r >>= 1;
            }
            op(sml, smr)
        }
        pub fn all_prod(&self) -> S {
            self.d[1]
        }
        pub fn apply(&mut self, idx: usize, val: F) {
            assert!(idx < self.n);
            let idx = idx + self.size;
            for i in (0..self.log+1).skip(1).rev() {
                self.push(idx >> i);
            }
            let mapping = self.mapping;
            self.d[idx] = mapping(val, self.d[idx]);
            for i in 1..=self.log {
                self.update(idx >> i);
            }
        }
        pub fn apply_range(&mut self, l: usize, r: usize, val: F) {
            assert!(l <= r && r <= self.n);
            // eprintln!("l: {}, r: {}", l, r);
            if l == r {
                return;
            }
            let l = l + self.size;
            let r = r + self.size;
            // eprintln!("l: {}, r: {}", l, r);
            for i in (0..self.log+1).skip(1).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r-1) >> i);
                }
            }
            {
                let mut l = l;
                let mut r = r;
                while l < r {
                    if (l & 1) != 0 {
                        self.all_apply(l, val);
                        l += 1;
                    }
                    if (r & 1) != 0 {
                        r -= 1;
                        self.all_apply(r, val);
                    }
                    l >>= 1;
                    r >>= 1;
                }
            }
            // eprintln!("l: {}, r: {}", l, r);
            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r-1) >> i);
                }
            }
        }
    
        fn update(&mut self, idx: usize) {
            let op = self.op;
            self.d[idx] = op(self.d[idx*2], self.d[idx*2 + 1]);
        }
        fn all_apply(&mut self, idx: usize, val: F) {
            let mapping = self.mapping;
            self.d[idx] = mapping(val, self.d[idx]);
            if idx < self.size {
                let composition = self.composition;
                self.lz[idx] = composition(val, self.lz[idx]);
            }
        }
        fn push(&mut self, idx: usize) {
            self.all_apply(idx*2, self.lz[idx]);
            self.all_apply(idx*2 + 1, self.lz[idx]);
            let id = self.id;
            self.lz[idx] = id();
        }
    }
    impl<S, F> std::fmt::Debug for LazySegtree<S, F>
    where
        S: Copy + Clone + std::fmt::Debug,
        F: Copy + Clone + std::fmt::Debug
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut d = vec![];
            let mut lz = vec![];
            let mut l = 1;
            let mut r = 2;
            while r < self.d.len() {
                let mut nd = vec![];
                let mut nlz = vec![];
                for i in l..r {
                    nd.push(self.d[i]);
                    nlz.push(self.lz[i]);
                }
                d.push(nd);
                lz.push(nlz);
                l <<= 1;
                r <<= 1;
            }
            write!(f, "d: {:?}\nlz: {:?}", d, lz)
        }
    }
}
use segtree::LazySegtree;

fn main() {
    let (n, q) = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut ws = buf.split_ascii_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let q: usize = ws.next().unwrap().parse().unwrap();
        (n, q)
    };
    
    let op = |l, r| std::cmp::min(l, r);
    let e = || 111222333444555666i64;
    let id = || 0i64;
    let mapping = |l, r| l + r;
    let composition = |l, r| l + r;
    let mut st = LazySegtree::from_vec(&vec![0i64; n], op, e, id, mapping, composition);
    for _ in 0..q {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut ws = buf.split_ascii_whitespace();

        let t: usize = ws.next().unwrap().parse().unwrap();
        if t == 0 {
            let (s, t, x) = {
                (ws.next().unwrap().parse::<usize>().unwrap(), ws.next().unwrap().parse::<usize>().unwrap(), ws.next().unwrap().parse::<i64>().unwrap())
            };
            st.apply_range(s, t+1, x);
            // eprintln!("{:?}", st);
        } else {
            let (s, t) = {
                (ws.next().unwrap().parse::<usize>().unwrap(), ws.next().unwrap().parse::<usize>().unwrap())
            };
            println!("{}", st.prod(s, t+1));
            // eprintln!("{:?}", st);
        }
    }
}
