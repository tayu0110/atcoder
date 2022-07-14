#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {t: usize};

    let mut resv = vec![];

    for _ in 0..t {
        input! {b: i64, k: i64, mut sx: i64, mut sy: i64, mut gx: i64, mut gy: i64};

        sx -= b;
        sy -= b;
        gx -= b;
        gy -= b;

        let greedy = ((sx - gx).abs() + (sy - gy).abs()) * k;
        let res1 = sx.abs() * k + (sy - gy).abs() + gx.abs() * k;
        let res2 = sy.abs() * k + (sx - gx).abs() + gy.abs() * k;
        let res3 = sx.abs() * k + sy.abs() + gx.abs() + gy.abs() * k;
        let res4 = sy.abs() * k + sx.abs() + gy.abs() + gx.abs() * k;
        let mut res = greedy;
        res = std::cmp::min(res, res1);
        res = std::cmp::min(res, res2);
        res = std::cmp::min(res, res3);
        res = std::cmp::min(res, res4);

        // println!("{}", res);
        resv.push(res);
    }

    for v in resv {
        println!("{}", v);
    }
}
