use proconio::*;

// fn solve(
//     now: usize,
//     m: usize,
//     c: &[i64],
//     a: &[i64],
//     can_use: &mut [bool],
//     l: &Vec<Vec<i64>>,
//     num_l: &mut [[i32; 6]],
// ) -> i64 {
//     if now == c.len() {
//         let mut max = vec![0; num_l.len()];
//         let mut res = 0;
//         for i in 0..num_l.len() {
//             for j in 0..m {
//                 if can_use[j] {
//                     max[i] = max[i].max(l[j][i]);
//                 }
//             }
//         }

//         for i in 0..m {
//             if can_use[i] {
//                 res += a[i];
//             }
//         }
//     }

//     let cnt = can_use.iter().filter(|&&f| f).count();
//     if cnt == 0 {
//         return 0;
//     } else if cnt == 1 {
//         let mut res = 0;
//         for i in 0..m {
//             if can_use[i] {
//                 for (j, v) in l[i].iter().enumerate() {
//                     res -= c[j] * v;
//                 }
//                 res += a[i];
//             }
//         }

//         return res;
//     }

//     let mut res = solve(now + 1, m, c, a, can_use, l, num_l);
//     for i in (1..=5).rev() {
//         for (j, v) in l.iter().enumerate() {
//             if can_use[j] && v[now] == i {
//                 can_use[j] = false;
//                 for (k, &v) in v.iter().enumerate() {
//                     num_l[k][v as usize] -= 1;
//                 }
//             }
//         }

//         res = res.max(solve(now, m, c, a, can_use, l, num_l));
//     }

//     res
// }

fn main() {
    input! {n: usize, m: usize, _c: [usize; n], _a: [usize; m], l: [[usize; n]; m]}

    let mut num_l = vec![[0; 6]; n];
    for v in &l {
        for (i, &v) in v.into_iter().enumerate() {
            num_l[i][v] += 1;
        }
    }
}
