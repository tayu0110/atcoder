use itertools::Itertools;
use proconio::*;

fn rotate(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            res[i][j] = a[3 - j][i]
        }
    }
    res
}

fn truncate(a: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut a = a.clone();
    while !a.is_empty() {
        if a.last().unwrap().iter().all(|&c| c == '.') {
            a.pop();
        } else {
            break;
        }
    }
    while !a.is_empty() {
        if a[0].iter().all(|&c| c == '.') {
            a.remove(0);
        } else {
            break;
        }
    }
    if a.is_empty() {
        return a;
    }
    let len = a.len();
    while !a[0].is_empty() {
        if (0..len).all(|i| *a[i].last().unwrap() == '.') {
            a.iter_mut().for_each(|a| {
                a.pop().unwrap();
            });
        } else {
            break;
        }
    }
    while !a[0].is_empty() {
        if (0..len).all(|i| a[i][0] == '.') {
            a.iter_mut().for_each(|a| {
                a.remove(0);
            });
        } else {
            break;
        }
    }
    a
}

fn solve(
    s: &Vec<Vec<char>>,
    t: &Vec<Vec<char>>,
    u: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    k: usize,
    l: usize,
    m: usize,
    n: usize,
) -> bool {
    let mut buf = vec![vec!['.'; 12]; 12];
    for ni in i..i + s.len() {
        if !s.is_empty() {
            for nj in j..j + s[0].len() {
                buf[ni][nj] = s[ni - i][nj - j];
            }
        }
    }
    for nk in k..k + t.len() {
        if !t.is_empty() {
            for nl in l..l + t[0].len() {
                if buf[nk][nl] == '#' {
                    if t[nk - k][nl - l] == '#' {
                        return false;
                    }
                    continue;
                }
                buf[nk][nl] = t[nk - k][nl - l];
            }
        }
    }
    for nm in m..m + u.len() {
        if !u.is_empty() {
            for nn in n..n + u[0].len() {
                if buf[nm][nn] == '#' {
                    if u[nm - m][nn - n] == '#' {
                        return false;
                    }
                    continue;
                }
                buf[nm][nn] = u[nm - m][nn - n];
            }
        }
    }

    let buf = truncate(&buf);
    if buf.len() != 4 || buf[0].len() != 4 {
        return false;
    }

    if buf.iter().flatten().any(|&c| c == '.') {
        return false;
    }

    true
}

fn main() {
    input! {mut s: [marker::Chars; 4], mut t: [marker::Chars; 4], mut u: [marker::Chars; 4]}

    for _ in 0..4 {
        for _ in 0..4 {
            for _ in 0..4 {
                {
                    let (s, t, u) = (truncate(&s), truncate(&t), truncate(&u));
                    let p = vec![&s, &t, &u];
                    for v in (0..3).permutations(3) {
                        let (s, t, u) = (p[v[0]], p[v[1]], p[v[2]]);
                        for k in 0..=8 {
                            for l in 0..=8 {
                                for m in 0..=8 {
                                    for n in 0..=8 {
                                        if solve(s, t, u, 0, 0, k, l, m, n) {
                                            println!("Yes");
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                u = rotate(u);
            }

            t = rotate(t);
        }

        s = rotate(s);
    }

    println!("No");
}
