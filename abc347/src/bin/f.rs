use std::collections::BinaryHeap;

use proconio::*;

fn solve(n: usize, m: usize, a: Vec<Vec<usize>>) -> usize {
    let mut cum = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        cum[i + 1][1..].copy_from_slice(&a[i][..]);
    }
    for i in 0..n {
        for j in 0..=n {
            cum[i + 1][j] += cum[i][j];
        }
    }
    for j in 0..n {
        for i in 0..=n {
            cum[i][j + 1] += cum[i][j];
        }
    }

    let mut forward = vec![vec![0; n]; n];
    for i in (0..n).take_while(|i| i + m <= n) {
        for j in (0..n).take_while(|i| i + m <= n) {
            forward[i][j] = cum[i + m][j + m] + cum[i][j] - cum[i + m][j] - cum[i][j + m];
        }
    }
    let mut backward = forward.clone();
    for i in (0..n).take_while(|i| i + m <= n) {
        for j in 1..n {
            forward[i][j] = forward[i][j - 1].max(forward[i][j]);
        }
        for j in (1..n).rev() {
            backward[i][j - 1] = backward[i][j].max(backward[i][j - 1]);
        }
    }

    let mut res = 0;
    for col in (m..n).take_while(|col| col + m <= n) {
        let (mut l, mut r, mut d) = (forward[0][col - m], backward[0][col], BinaryHeap::new());
        for i in m..n {
            d.push((forward[i][n - 1], i));
        }

        for i in (0..n).take_while(|i| i + m + m <= n) {
            l = l.max(forward[i][col - m]);
            r = r.max(backward[i][col]);
            while let Some(&(_, j)) = d.peek() {
                if j < i + m {
                    d.pop();
                } else {
                    break;
                }
            }

            if d.is_empty() {
                break;
            }
            res = res.max(l + r + d.peek().unwrap().0);
            // eprintln!("col: {col}, i: {i}, l: {l}, r: {r}, res: {res}");
        }
    }

    res
}

fn rot(n: usize, mut a: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    for layer in 0..n / 2 {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let j = last - i + first;
            let tmp = a[first][i];
            a[first][i] = a[j][first];
            a[j][first] = a[last][j];
            a[last][j] = a[i][last];
            a[i][last] = tmp;
        }
    }
    a
}

fn main() {
    input! {n: usize, m: usize, mut a: [[usize; n]; n]}

    let mut b = a.clone();
    let mut res = solve(n, m, b.clone());
    for _ in 0..3 {
        b = rot(n, b);
        res = res.max(solve(n, m, b.clone()));
    }

    assert_eq!(a, b);

    println!("{res}")
}
