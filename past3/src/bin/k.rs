use ds::LinkCutTree;
use iolib::*;

fn main() {
    scan!(n: usize, q: usize, query: [(usize, usize, usize); q]);

    let mut lct = <LinkCutTree>::new(n * 2);
    let mut top = (0..n).collect::<Vec<_>>();
    for i in 0..n {
        lct.link(i + n, i).unwrap();
    }

    for (f, t, x) in query {
        let ot = top[f - 1];
        top[f - 1] = lct.parent(x - 1).unwrap();
        lct.cut(x - 1);
        lct.link(top[t - 1], x - 1).unwrap();
        top[t - 1] = ot;
    }

    for i in 0..n {
        let r = lct.root(i);
        putln!(r - n + 1);
    }
}
