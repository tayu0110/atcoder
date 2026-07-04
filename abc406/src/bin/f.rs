use cpio::*;
use ds::{EulerTourTree, MapMonoid};

struct T;
impl MapMonoid for T {
    type M = u32;
    type Act = ();

    fn e() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    scan! {n: usize, mut e: [(usize, usize); n - 1], q: usize}

    let mut ett = EulerTourTree::<T>::new(n);
    for (u, v) in e.iter_mut() {
        *u -= 1;
        *v -= 1;
        ett.link(*u, *v).ok();
    }
    for i in 0..n {
        ett.set(i, 1);
    }

    let mut sum = n as u32;
    for _ in 0..q {
        scan! {ty: usize}

        if ty == 1 {
            scan! {x: usize, w: u32}
            ett.update_by(x - 1, |l| l + w);
            sum += w;
        } else {
            scan! {y: usize}
            let (u, v) = e[y - 1];

            let f = ett.fold_subtree(v, u).unwrap();
            let g = sum - f;
            putln!(f.abs_diff(g));
        }
    }
}
