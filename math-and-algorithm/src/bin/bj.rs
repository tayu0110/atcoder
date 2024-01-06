// TODO
// use proconio::*;
// use segtree::SegmentTree;

fn main() {
    // input! {n: usize, mut p: [(i64, i64); n]}
    // p.iter_mut().for_each(|(l, r)| {
    //     *l += 100000;
    //     *r += 100000;
    // });
    // p.sort();

    // let mut pst = SegmentTree::new(200001, (0i64, 0i64), |&l, &r| (l.0 + r.0, l.1 + r.1));
    // let mut mst = pst.clone();

    // let mut res = 0;
    // for (x, y) in p {
    //     let (c, p) = pst.foldl(0, y as usize + 1);
    //     res += (x + y) * c - p;
    //     let (c, m) = mst.foldl(y as usize + 1, 200001);
    //     res += (x - y) * c - m;

    //     pst.update_by(y as usize, (1, x + y), |&l, &r| (l.0 + r.0, l.1 + r.1));
    //     mst.update_by(y as usize, (1, x - y), |&l, &r| (l.0 + r.0, l.1 + r.1));
    // }

    // println!("{res}")
}
