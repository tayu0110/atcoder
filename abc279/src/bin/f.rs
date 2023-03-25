#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, q: usize}

    let mut uf = unionfind::UnionFind::new(n + q + 10);
    let mut used_backet = n;

    // 箱番号とUFのNode番号の対応付け
    let mut box_num = (0..=n).collect::<Vec<_>>();
    // ボール番号とUFのNode番号の対応付け
    let mut ball_num = (0..=n).collect::<Vec<_>>();
    // UFの根から箱番号を辿る
    let mut root_to_box = (0..n + q + 10).collect::<Vec<_>>();

    let mut res = vec![];
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, y: usize}
            uf.merge(box_num[x], box_num[y]);
            box_num[x] = uf.root(box_num[x]);
            root_to_box[box_num[x]] = x;
            used_backet += 1;
            box_num[y] = used_backet;
            root_to_box[uf.root(used_backet)] = y;
        } else if t == 2 {
            input! {x: usize}
            used_backet += 1;
            ball_num.push(used_backet);
            uf.merge(box_num[x], used_backet);
            box_num[x] = uf.root(used_backet);
            root_to_box[box_num[x]] = x;
        } else {
            input! {x: usize}
            let root = uf.root(ball_num[x]);
            res.push(root_to_box[root]);
        }

        // eprintln!("used_backet: {}", used_backet);
        // eprintln!("box_num: {:?}", box_num);
        // eprintln!("ball_num: {:?}", ball_num);
        // eprintln!("root_to_box: {:?}", root_to_box);
    }

    for res in res {
        println!("{}", res);
    }
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>,
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind {
                tree: vec![-1; size],
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            if self.tree[index] < 0 {
                index
            } else {
                self.tree[index] = self.root(self.tree[index] as usize) as i32;
                self.tree[index] as usize
            }
        }
        pub fn size(&mut self, index: usize) -> usize {
            let root = self.root(index);
            -self.tree[root] as usize
        }
        pub fn is_same(&mut self, left: usize, right: usize) -> bool {
            self.root(left) == self.root(right)
        }
        pub fn merge(&mut self, left: usize, right: usize) -> bool {
            let (mut rl, mut rr) = (self.root(left), self.root(right));
            if rl == rr {
                return false;
            }
            if self.tree[rl] > self.tree[rr] {
                std::mem::swap(&mut rl, &mut rr);
            }
            self.tree[rl] += self.tree[rr];
            self.tree[rr] = rl as i32;
            true
        }
    }
}
