use std::{
    isize,
    ops::{Bound, Range, RangeBounds},
};

use num::rational::Ratio;
use proconio::*;
use segtree::Monoid;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, lines: [(isize, isize, isize); n]}

        let mut lct = LiChaoTree::new(-10..1000_100_100);
        for (a, b, c) in lines {
            lct.add_segment((a, b, c), 0..=1000_000_000);
        }

        let mut res = 0;
    }
}

fn convert_range_isize(min: isize, max: isize, range: impl RangeBounds<isize>) -> Range<isize> {
    let l = match range.start_bound() {
        Bound::Included(l) => *l,
        Bound::Unbounded => min,
        Bound::Excluded(l) => l - 1,
    };
    let r = match range.end_bound() {
        Bound::Included(r) => r + 1,
        Bound::Excluded(r) => *r,
        Bound::Unbounded => max,
    };
    Range { start: l, end: r }
}

struct Node<T, L> {
    left: u32,
    right: u32,
    val: T,
    _lazy: L,
}

impl<T, L> Node<T, L> {
    fn new(val: T, lazy: L) -> Self {
        Self {
            left: u32::MAX,
            right: u32::MAX,
            val,
            _lazy: lazy,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Zst;

impl Monoid for Zst {
    type M = Self;
    fn id() -> Self::M {
        Zst
    }
    fn op(_: &Self::M, _: &Self::M) -> Self::M {
        Zst
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Affine {
    a: isize,
    b: isize,
    c: isize,
}

impl Affine {
    fn eval(self, x: isize) -> Ratio<isize> {
        let r0 = Ratio::new(-self.a, self.b) * Ratio::new(x, 1);
        let r1 = Ratio::new(self.c, 1);
        r0 + r1
    }
}

impl From<(isize, isize, isize)> for Affine {
    fn from(value: (isize, isize, isize)) -> Self {
        Affine {
            a: value.0,
            b: value.1,
            c: value.2,
        }
    }
}

type LiChaoTreeNode = Node<Affine, Zst>;

pub struct LiChaoTree {
    range: Range<isize>,
    nodes: Vec<LiChaoTreeNode>,
}

impl LiChaoTree {
    pub fn new(range: impl RangeBounds<isize>) -> Self {
        Self {
            range: convert_range_isize(isize::MIN, isize::MAX, range),
            nodes: vec![LiChaoTreeNode::new((0, 0, isize::MIN).into(), Zst)],
        }
    }

    pub fn len(&self) -> usize {
        self.range.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn create_node(&mut self) -> u32 {
        let res = self.nodes.len();
        let node = LiChaoTreeNode::new((0, 0, isize::MAX).into(), Zst);

        self.nodes.push(node);
        res as u32
    }

    fn do_add_line(&mut self, val: Affine, l: isize, r: isize, now: usize) {
        if r - l == 1 {
            if val.eval(l) < self.nodes[now].val.eval(l) {
                self.nodes[now].val = val;
            }
            return;
        }

        let (nl, nr) = (val.eval(l), val.eval(r));
        let (pl, pr) = (self.nodes[now].val.eval(l), self.nodes[now].val.eval(r));

        if nl <= pl && nr <= pr {
            self.nodes[now].val = val;
            return;
        }

        if nl >= pl && nr >= pr {
            return;
        }

        let mid = (r + l) >> 1;

        if self.nodes[now].left == u32::MAX {
            let left = self.create_node();
            self.nodes[now].left = left;
            self.nodes[left as usize].val = val;
            return;
        }
        if self.nodes[now].right == u32::MAX {
            let right = self.create_node();
            self.nodes[now].right = right;
            self.nodes[right as usize].val = val;
            return;
        }

        self.do_add_line(val, l, mid, self.nodes[now].left as usize);
        self.do_add_line(val, mid, r, self.nodes[now].right as usize);
    }

    #[inline]
    pub fn add_line(&mut self, val: impl Into<Affine>) {
        self.do_add_line(val.into(), self.range.start, self.range.end, 0);
    }

    fn do_add_segment(
        &mut self,
        val: Affine,
        start: isize,
        end: isize,
        l: isize,
        r: isize,
        now: usize,
    ) {
        if start <= l && r <= end {
            self.do_add_line(val, l, r, now);
            return;
        }

        if r <= start || end <= l {
            return;
        }

        let mid = (r + l) >> 1;
        if start < mid {
            if self.nodes[now].left == u32::MAX {
                self.nodes[now].left = self.create_node();
            }
            self.do_add_segment(val, start, end, l, mid, self.nodes[now].left as usize);
        }
        if mid <= end {
            if self.nodes[now].right == u32::MAX {
                self.nodes[now].right = self.create_node();
            }
            self.do_add_segment(val, start, end, mid, r, self.nodes[now].right as usize);
        }
    }

    #[inline]
    pub fn add_segment(&mut self, val: impl Into<Affine>, range: impl RangeBounds<isize>) {
        let Range { start, end } = convert_range_isize(self.range.start, self.range.end, range);

        self.do_add_segment(val.into(), start, end, self.range.start, self.range.end, 0);
    }

    fn do_evaluate(&self, x: isize, l: isize, r: isize, now: u32) -> Ratio<isize> {
        if l >= r || now == u32::MAX {
            return Ratio::new(isize::MAX, 1);
        }

        let mut res = self.nodes[now as usize].val.eval(x);
        if r - l == 1 {
            return res;
        }

        let mid = (r + l) >> 1;
        if x < mid {
            res = res.min(self.do_evaluate(x, l, mid, self.nodes[now as usize].left));
        } else {
            res = res.min(self.do_evaluate(x, mid, r, self.nodes[now as usize].right));
        }

        res
    }

    pub fn evaluate(&self, x: isize) -> Ratio<isize> {
        self.do_evaluate(x, self.range.start, self.range.end, 0)
    }

    fn do_get_line(&self, x: isize, l: isize, r: isize, now: u32) -> (Ratio<isize>, Affine) {
        if l >= r || now == u32::MAX {
            return (
                Ratio::new(isize::MAX, 1),
                Affine {
                    a: 0,
                    b: 0,
                    c: isize::MAX,
                },
            );
        }

        let mut res = self.nodes[now as usize].val.eval(x);
        if r - l == 1 {
            return (res, self.nodes[now as usize].val);
        }

        let mid = (r + l) >> 1;
        let mut affine = Affine {
            a: 0,
            b: 0,
            c: isize::MAX,
        };
        if x < mid {
            let (evaled, line) = self.do_get_line(x, l, mid, self.nodes[now as usize].left);
            if res > evaled {
                res = evaled;
                affine = line;
            }
        } else {
            let (evaled, line) = self.do_get_line(x, l, mid, self.nodes[now as usize].right);
            if res > evaled {
                res = evaled;
                affine = line;
            }
        }

        (res, affine)
    }

    pub fn get_line(&self, x: isize) -> Affine {
        self.do_get_line(x, self.range.start, self.range.end, 0).1
    }
}
