use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}

    let ord = |c: char| c as usize - b'a' as usize;
    let mut a = vec![std::usize::MAX; 26];
    let mut b = vec![std::usize::MAX; 26];
    for (cs, ct) in s.into_iter().zip(t) {
        if a[ord(cs)] != std::usize::MAX && a[ord(cs)] != ord(ct) {
            println!("No");
            return;
        }
        if b[ord(ct)] != std::usize::MAX && b[ord(ct)] != ord(cs) {
            println!("No");
            return;
        }

        a[ord(cs)] = ord(ct);
        b[ord(ct)] = ord(cs);
    }

    if a.iter().any(|&a| a == std::usize::MAX) {
        println!("Yes");
        return;
    }

    for i in 0..26 {
        for j in i + 1..26 {
            if i == a[j] {
                a.swap(i, j);
                break;
            }
        }
    }

    if a.iter().enumerate().all(|(i, &a)| i == a) {
        println!("Yes")
    } else {
        println!("No")
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
