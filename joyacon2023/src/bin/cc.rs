use proconio::input;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}
    p.sort();

    let mut res = vec![];
    let (mut l, mut r) = p[0];
    for (x, y) in p.into_iter().skip(1) {
        if x <= r {
            r = std::cmp::max(r, y);
        } else {
            res.push((l, r));
            l = x;
            r = y;
        }
    }
    res.push((l, r));

    for (x, y) in res {
        println!("{} {}", x, y);
    }
}
