use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n], c: [usize; n]}

    let mut cum_b = vec![0; n + 1];
    for i in 0..n {
        cum_b[i + 1] = cum_b[i] + b[i];
    }
    let mut cum_c = vec![0; n + 1];
    for i in (0..n).rev() {
        cum_c[i] = cum_c[i + 1] + c[i];
    }

    let mut nt = BinaryHeap::new();
    for i in 0..n {
        nt.push((cum_b[i] + cum_c[i], i));
    }

    let mut ret = 0;
    let mut sum = 0;
    for (i, a) in a.into_iter().enumerate().take(n - 2) {
        sum += a;
        while let Some(&(max, j)) = nt.peek() {
            if j <= i + 1 {
                nt.pop();
            } else {
                ret = ret.max(sum + max - cum_b[i + 1]);
                break;
            }
        }
    }

    println!("{ret}")
}
