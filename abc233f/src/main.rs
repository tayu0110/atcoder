use std::collections::{VecDeque};

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

fn dfs(now: usize, par: usize, target: usize, t: &Vec<Vec<(usize, usize)>>, res: &mut Vec<usize>, p: &Vec<usize>) -> bool {
    if p[now] == target {
        return true;
    }

    for (to, idx) in &t[now] {
        if *to == par {
            continue;
        }
        if dfs(*to, now, target, t, res, p) {
            res.push(*idx);
            return true;
        }
    }

    false
}

const INF: usize = 1usize << 60;

fn main() {
    input!(n: usize, p: [usize; n], m: usize, q: [(usize, usize); m]);

    let mut p = p.iter().map(|x| *x-1).collect::<Vec<usize>>();
    let q = q.iter().map(|(a, b)| (*a-1, *b-1)).collect::<Vec<(usize, usize)>>();
    let mut uf = UnionFind::new(n);
    let mut t = vec![vec![]; n];

    for (index, (a, b)) in q.iter().enumerate() {
        if !uf.is_same(*a, *b) {
            uf.merge(*a, *b);
            t[*a].push((*b, index));
            t[*b].push((*a, index));
        }
    }

    let mut nt = VecDeque::new();
    for (i, v) in t.iter().enumerate() {
        if v.len() == 0 && p[i] != i {
            println!("-1");
            std::process::exit(0);
        }
        if v.len() == 1 {
            nt.push_back(i);
        }
    }

    // eprintln!("{:?}", nt);

    let mut res = vec![];
    while !nt.is_empty() {
        let now = nt.pop_front().unwrap();

        let mut buf = vec![];
        if !dfs(now, INF, now, &t, &mut buf, &p) {
            println!("-1");
            std::process::exit(0);
        }

        // buf.reverse();
        // eprintln!("{:?}", buf);
        for v in buf {
            res.push(v);
            
            let (a, b) = q[v];
            p.swap(a, b);
        }

        if t[now].len() > 0 {
            let (par, _) = t[now][0];
            let k = {
                let mut ret = 0;
                for (i, (v, _)) in t[par].iter().enumerate() {
                    if *v == now {
                        ret = i;
                        break;
                    }
                }
                ret
            };
            t[par].remove(k);
            if t[par].len() <= 1 {
                nt.push_back(par);
            }
        }

        // eprintln!("{:?}", p);
        // eprintln!("{:?}", t);
    }

    println!("{}", res.len());
    for v in res {
        println!("{}", v+1);
    }
}