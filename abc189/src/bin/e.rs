#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(i64, i64); n], m: usize};

    let mut op = vec![];
    for _ in 0..m {
        input! {t: usize}
        let v = if t == 1 {
            vec![vec![0, 1, 0], vec![-1, 0, 0], vec![0, 0, 1]]
        } else if t == 2 {
            vec![vec![0, -1, 0], vec![1, 0, 0], vec![0, 0, 1]]
        } else if t == 3 {
            input! {p: i64}
            vec![vec![-1, 0, 2 * p], vec![0, 1, 0], vec![0, 0, 1]]
        } else {
            input! {p: i64}
            vec![vec![1, 0, 0], vec![0, -1, 2 * p], vec![0, 0, 1]]
        };

        if op.is_empty() {
            op.push(v);
        } else {
            let last = op.last().unwrap();
            let mut nop = vec![vec![0; 3], vec![0; 3], vec![0; 3]];
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        nop[i][j] += v[i][k] * last[k][j];
                    }
                }
            }
            op.push(nop);
        }
    }

    input! {q: usize, q: [(usize, usize); q]}
    for (a, b) in q {
        let (x, y) = p[b-1];
        if a == 0 {
            println!("{} {}", x, y);
            continue;
        }
        let op = &op[a-1];

        println!("{} {}", op[0][0] * x + op[0][1] * y + op[0][2], op[1][0] * x + op[1][1] * y + op[1][2]);
    }
}
