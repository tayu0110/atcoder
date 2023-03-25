use flow::Dinic;
use proconio::{input, marker::Chars};

fn main() {
    input! {h: usize, w: usize, a: [Chars; h]}

    // in: i, out: i+h*w
    let mut ff = Dinic::new(h * w * 2);
    let get_index = |i: usize, j: usize| i * w + j;
    let is_st = |c: char| c == 'S' || c == 'T';
    let (mut s, mut t) = (0, 0);

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                s = get_index(i, j);
            }
            if a[i][j] == 'T' {
                t = get_index(i, j);
            }
            if a[i][j] != '.' {
                for k in 0..h {
                    if k != i && is_st(a[i][j]) && is_st(a[k][j]) {
                        println!("-1");
                        return;
                    }
                    if k != i && a[k][j] != '.' {
                        ff.set_edge(get_index(i, j) + h * w, get_index(k, j), std::i32::MAX);
                    }
                }
                for k in 0..w {
                    if k != j && is_st(a[i][j]) && is_st(a[i][k]) {
                        println!("-1");
                        return;
                    }
                    if k != j && a[i][k] != '.' {
                        ff.set_edge(get_index(i, j) + h * w, get_index(i, k), std::i32::MAX);
                    }
                }
            }
        }
    }

    for i in 0..h * w {
        ff.set_edge(i, i + h * w, 1);
    }

    let res = ff.flow(s + h * w, t);
    println!("{}", res);
}
