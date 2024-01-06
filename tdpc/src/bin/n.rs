// TODO
// use proconio::*;
// use static_modint::{combination, Mod1000000007, StaticModint};

// type Modint = StaticModint<Mod1000000007>;

// fn calc_size(now: usize, par: usize, size: &mut [usize], t: &Vec<Vec<usize>>) -> usize {
//     let mut res = 1;
//     for &to in &t[now] {
//         if to != par {
//             res += calc_size(to, now, size, t);
//         }
//     }
//     size[now] = res;
//     res
// }

// fn rec(
//     now: usize,
//     par: usize,
//     size: &[usize],
//     t: &Vec<Vec<usize>>,
//     com: &impl Fn(u32, u32) -> Modint,
// ) -> Modint {
//     if t[now].len() == 1 && t[now][0] == par {
//         return Modint::one();
//     }

//     let mut res = Modint::one();
//     let mut rem = size[now] - 1;
//     for &to in &t[now] {
//         if to != par {
//             let r = rec(to, now, size, t, com);
//             res *= com(rem, size[to]) * r;
//             rem -= size[to];
//         }
//     }
//     res
// }

// fn solve(root: usize, t: &Vec<Vec<usize>>, com: &impl Fn(u32, u32) -> Modint) -> Modint {
//     let n = t.len();
//     let mut size = vec![0; n];
//     calc_size(root, root, &mut size, t);
//     rec(root, root, &size, t, com)
// }

fn main() {
    // input! {n: usize, e: [(usize, usize); n-1]}

    // let mut t = vec![vec![]; n];
    // for (a, b) in e {
    //     t[a - 1].push(b - 1);
    //     t[b - 1].push(a - 1);
    // }

    // let mut res = Modint::zero();
    // let com = combination(n as u32);
    // for i in 0..n {
    //     res += solve(i, &t, &com);
    // }

    // println!("{}", res / Modint::raw(2))
}
