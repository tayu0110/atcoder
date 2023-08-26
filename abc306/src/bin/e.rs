use proconio::*;
// use rand::{thread_rng, Rng};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    input! {n: usize, k: usize, q: usize}
    input! {p: [(usize, usize); q]}
    // let (n, k, q) = (10, 5, 10);
    // let mut rng = thread_rng();
    // let mut p = vec![];
    // for _ in 0..q {
    //     let x: usize = rng.gen_range(1, n + 1);
    //     let y: usize = rng.gen_range(0, 10);
    //     p.push((x, y));
    // }
    // eprintln!("n: {}, k: {}, q: {}, p: {:?}", n, k, q, p);

    if k == n {
        let mut a = vec![0; n];
        let mut sum = 0;
        for (x, y) in p {
            sum -= a[x - 1];
            sum += y;
            a[x - 1] = y;
            println!("{}", sum);
        }

        return;
    }

    let mut max_set = HashSet::new();
    let mut max = BinaryHeap::new();
    let mut min = BinaryHeap::new();
    for i in 0..k {
        max_set.insert(i);
        max.push(Reverse((0, i)));
    }
    for i in k..n {
        min.push((0, i));
    }

    let mut sum = 0;
    let mut a = vec![0; n];
    for (x, y) in p {
        let x = x - 1;
        if max_set.contains(&x) {
            sum -= a[x];
            while let Some((d, i)) = min.pop() {
                if a[i] != d || max_set.contains(&i) || i == x {
                    continue;
                }
                if d > y {
                    min.push((y, x));
                    max.push(Reverse((d, i)));
                    max_set.insert(i);
                    max_set.remove(&x);
                    sum += d;
                } else {
                    max.push(Reverse((y, x)));
                    min.push((d, i));
                    sum += y;
                }
                break;
            }
        } else {
            while let Some(Reverse((d, i))) = max.pop() {
                if a[i] != d || !max_set.contains(&i) || i == x {
                    continue;
                }

                if d < y {
                    min.push((d, i));
                    max.push(Reverse((y, x)));
                    max_set.insert(x);
                    max_set.remove(&i);
                    sum += y;
                    sum -= d;
                } else {
                    min.push((y, x));
                    max.push(Reverse((d, i)));
                }
                break;
            }
        }
        a[x] = y;

        println!("{}", sum);

        // let mut s = a.clone();
        // s.sort();
        // s.reverse();
        // eprintln!(
        //     "a: {:?}, max_set: {:?}, max: {:?}, min: {:?}",
        //     a, max_set, max, min
        // );
        // assert_eq!(s[..k].iter().sum::<usize>(), sum);
    }
}
