// TODO
// use itertools::Itertools;
// use proconio::*;
// use segtree::SegmentTree;

fn main() {
    // input! {n: usize, k: usize, c: usize, s: marker::Chars}

    // let mut st = SegmentTree::new(n, 0, |&l, &r| l.max(r));
    // let mut t = vec![vec![]; k];
    // for (i, s) in s.into_iter().enumerate() {
    //     if s == 'x' {
    //         continue;
    //     }
    //     if i <= c {
    //         t[0].push(i);
    //     } else {
    //         let max = st.foldl(0, i - c);
    //         st.set(i, max + 1);
    //         if max + 1 < k {
    //             t[max + 1].push(i);
    //         }
    //     }
    // }
    // eprintln!("t: {t:?}");

    // let mut res = vec![];
    // let mut prev = t.pop().unwrap();
    // if prev.len() == 1 {
    //     res.push(prev[0] + 1);
    // }
    // while let Some(mut now) = t.pop() {
    //     let len = now.len();
    //     for i in (0..len).rev() {
    //         let t = now[i];
    //         if let Some(&last) = prev.iter().last() {
    //             if t + c < last {
    //                 continue;
    //             }
    //         }
    //         now.swap_remove(i);
    //     }
    //     if now.len() == 1 {
    //         res.push(now[0] + 1);
    //     }
    //     now.sort();
    //     prev = now;
    // }

    // res.sort();
    // println!("{}", res.iter().join("\n"))
}
