use itertools::Itertools;
use proconio::*;

fn main() {
    input! {c: [[usize; 3]; 3]}

    let mut t = vec![];
    for i in 0..3 {
        for j in 0..3 {
            t.push((i, j));
        }
    }

    let mut cnt = 0;
    for v in (0..9).permutations(9) {
        let mut check = vec![vec![0; 3]; 3];
        for (i, v) in v.into_iter().enumerate() {
            let (y, x) = t[v];
            check[y][x] = i;
        }

        let mut bad = false;
        'K: for i in 0..3 {
            for j in 0..3 {
                for (dy, dx) in [(0usize, 1usize),
                    (1, 0),
                    (0, !0),
                    (!0, 0),
                    (1, 1),
                    (1, !0),
                    (!0, 1),
                    (!0, !0)] {
                    let mut p = vec![];
                    let (mut y, mut x) = (i, j);
                    for _ in 0..3 {
                        if y >= 3 || x >= 3 {
                            break;
                        }

                        p.push((check[y][x], c[y][x]));
                        y = y.wrapping_add(dy);
                        x = x.wrapping_add(dx);
                    }

                    if p.len() < 3 {
                        continue;
                    }

                    p.sort();
                    if p[0].1 == p[1].1 && p[2].1 != p[0].1 {
                        bad = true;
                        break 'K;
                    }
                }
            }
        }

        if !bad {
            cnt += 1;
        }
    }

    let mut res = cnt as f64;
    for i in 1..10 {
        res /= i as f64;
    }

    println!("{res}")
}

// use itertools::Itertools;
// use proconio::*;

// fn main() {
//     input! {c: [[usize; 3]; 3]}

//     let mut t = vec![];
//     for i in 0..3 {
//         for j in 0..3 {
//             t.push((i, j));
//         }
//     }

//     let mut cnt = 0usize;
//     for v in (0..9).permutations(9) {
//         let mut checked = vec![vec![false; 3]; 3];
//         let mut ok = true;
//         for &v in &v {
//             let (y, x) = t[v];
//             let tar = c[y][x];

//             if y == 1 && x == 1 {
//                 if checked[0][1] && checked[2][1] && c[0][1] == c[2][1] && c[0][1] != tar {
//                     ok = false;
//                     break;
//                 }
//                 if checked[1][0] && checked[1][2] && c[1][0] == c[1][2] && c[1][0] != tar {
//                     ok = false;
//                     break;
//                 }
//                 if checked[0][0] && checked[2][2] && c[0][0] == c[2][2] && c[0][0] != tar {
//                     ok = false;
//                     break;
//                 }
//                 if checked[0][2] && checked[2][0] && c[0][2] == c[2][0] && c[0][2] != tar {
//                     ok = false;
//                     break;
//                 }
//                 checked[y][x] = true;
//                 continue;
//             }

//             let mut bad = false;

//             for (dx, dy) in vec![
//                 (0usize, 1usize),
//                 (1, 0),
//                 (0, !0),
//                 (!0, 0),
//                 (1, 1),
//                 (1, !0),
//                 (!0, 1),
//                 (!0, !0),
//             ] {
//                 let mut d = 0;
//                 let (mut nr, mut nc) = (y, x);
//                 let mut l = vec![false; 10];
//                 for _ in 0..2 {
//                     nr = nr.wrapping_add(dy);
//                     nc = nc.wrapping_add(dx);

//                     if nr >= 3 || nc >= 3 || !checked[nr][nc] {
//                         break;
//                     }

//                     d += 1;
//                     l[c[nr][nc]] = true;
//                 }

//                 if d == 2 && l.iter().filter(|&&f| f).count() == 1 && !l[tar] {
//                     bad = true;
//                     break;
//                 }
//             }

//             if bad {
//                 ok = false;
//                 break;
//             }

//             checked[y][x] = true;
//         }

//         if ok {
//             // assert!(checked.into_iter().flatten().all(|f| f));
//             cnt += 1;
//         }
//     }

//     let mut res = cnt as f64;
//     for i in 1..=9 {
//         res /= i as f64;
//     }

//     println!("{}", res)
// }
