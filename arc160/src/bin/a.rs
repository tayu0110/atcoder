use itertools::Itertools;
use proconio::*;

fn rec(
    now: usize,
    k: usize,
    sum: &mut usize,
    pos: &[usize],
    a: &[usize],
) -> Option<(usize, usize)> {
    let n = pos.len();
    if now == n {
        return None;
    }

    for i in 0..a[now] {
        if pos[i] < now {
            continue;
        }

        *sum += 1;

        if *sum == k {
            return Some((now, pos[i]));
        }
    }

    if let Some(res) = rec(now + 1, k, sum, pos, a) {
        return Some(res);
    }
    *sum += 1;
    if *sum == k {
        return Some((now, now));
    }

    None
    // for i in a[now] + 1..n {
    //     if pos[i] < now {
    //         continue;
    //     }

    //     *sum += 1;
    //     if *sum == k {
    //         return Some((now, pos[i]));
    //     }
    // }

    // None
}

// fn naive(n: usize, k: usize, a: Vec<usize>) -> Vec<usize> {
//     let mut buf = vec![];
//     for i in 0..n {
//         for j in i..n {
//             let mut t = a.clone();
//             t[i..=j].reverse();
//             buf.push(t);
//         }
//     }

//     buf.sort();
//     eprintln!("buf: {:?}", buf);
//     let mut res = buf[k - 1].clone();
//     res.iter_mut().for_each(|v| *v += 1);
//     res
// }

fn main() {
    // let mut rng = thread_rng();
    // let n: usize = rng.gen_range(2, 100);
    // let k: usize = rng.gen_range(1, n * (n + 1) / 2);
    // let mut a = {
    //     let mut rem = (1..=n).collect::<Vec<_>>();
    //     let mut buf = vec![];
    //     for _ in 0..n {
    //         let index = rng.gen_range(0, rem.len());
    //         buf.push(rem[index]);
    //         rem.remove(index);
    //     }
    //     buf
    // };
    // eprintln!("n: {}, k: {}, a: {:?}", n, k, a);

    input! {n: usize, k: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut pos = vec![0; n];
    for i in 0..n {
        pos[a[i]] = i;
    }

    let mut sum = 0;
    // let b = naive(n, k, a.clone());
    let mut res = if let Some((l, r)) = rec(0, k, &mut sum, &pos, &a) {
        a[l..=r].reverse();
        a
    } else {
        'base: for i in (0..n).rev() {
            for p in a[i] + 1..n {
                if i < pos[p] {
                    sum += 1;
                    if sum == k {
                        a[i..=pos[p]].reverse();
                        break 'base;
                    }
                }
            }
        }
        a
    };
    res.iter_mut().for_each(|v| *v += 1);

    // assert_eq!(res, b);

    println!("{}", res.iter().join(" "));
}
