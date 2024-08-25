use proconio::*;

fn trim(a: &mut Vec<Vec<char>>) {
    let len = a[0].len();
    while !a.is_empty() {
        if a[0].iter().all(|v| *v == '.') {
            a.remove(0);
        } else {
            break;
        }
    }
    while let Some(v) = a.pop() {
        if v != vec!['.'; len] {
            a.push(v);
            break;
        }
    }

    if a.is_empty() {
        return;
    }

    while !a.is_empty() {
        if a.iter().all(|v| v[0] == '.') {
            a.iter_mut().for_each(|v| {
                v.remove(0);
            });
        } else {
            break;
        }
    }
}

fn main() {
    input! {h: usize, _: usize, mut a: [marker::Chars; h], h: usize, _: usize, mut b: [marker::Chars; h], h: usize, w: usize, x: [marker::Chars; h]}
    trim(&mut a);
    trim(&mut b);

    for i in 0..h {
        for j in 0..w {
            for k in 0..h {
                for l in 0..w {
                    let mut ok = vec![vec![false; w]; h];
                    let mut good = true;
                    for (i, j, v) in [(i, j, &a), (k, l, &b)] {
                        let r = v.len();
                        let c = v[0].len();
                        let mut bad = false;

                        'base: for k in 0..r {
                            for l in 0..c {
                                if v[k][l] == '#' {
                                    if i + k >= h || j + l >= w {
                                        bad = true;
                                        break 'base;
                                    }

                                    if x[i + k][j + l] != '#' {
                                        bad = true;
                                        break 'base;
                                    }
                                }
                            }
                        }

                        if !bad {
                            for k in 0..r {
                                for l in 0..c {
                                    if v[k][l] == '#' {
                                        ok[i + k][j + l] = true;
                                    }
                                }
                            }
                        } else {
                            good = false;
                        }
                    }

                    for i in 0..h {
                        for j in 0..w {
                            if x[i][j] == '#' && !ok[i][j] {
                                good = false;
                            }
                        }
                    }

                    if good {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No")
}
