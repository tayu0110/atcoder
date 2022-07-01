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

fn main() {
    input!(h: i32, w: i32, s: [chars; h as usize]);

    let f = |x: i32, y: i32| { (y * w + x) as usize };
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    let mut uf = UnionFind::new((h * w) as usize);

    for i in 0..h {
        for j in 0..w {
            for k in 0..4 {
                let ny = i + dy[k];
                let nx = j + dx[k];

                if ny < 0 || ny >= h || nx < 0 || nx >= w {
                    continue;
                }

                if s[i as usize][j as usize] != s[ny as usize][nx as usize] {
                    uf.merge(f(j, i), f(nx, ny));
                }
            }
        }
    }

    let mut v = vec![vec![0usize; 2]; (h*w) as usize];

    for i in 0..h {
        for j in 0..w {
            let root = uf.root(f(j, i));
            if s[i as usize][j as usize] == '#' {
                v[root][0] += 1;
            } else {
                v[root][1] += 1;
            }
        }
    }

    let mut res = 0;
    for i in 0..(h*w) as usize {
        res += v[i][0] * v[i][1];
    }

    println!("{}", res);
}
