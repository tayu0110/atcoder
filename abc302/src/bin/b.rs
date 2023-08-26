use proconio::*;

fn main() {
    input! {h: usize, w: usize, s: [marker::Chars; h]}

    let t = vec!['s', 'n', 'u', 'k', 'e'];
    let mut u = t.clone();
    u.reverse();
    for i in 0..h {
        for j in 0..w {
            if j + 5 <= w {
                if s[i][j..j + 5].iter().zip(&t).all(|(i, j)| i == j) {
                    for j in j..j + 5 {
                        println!("{} {}", i + 1, j + 1);
                    }
                    return;
                }

                if s[i][j..j + 5].iter().zip(&u).all(|(i, j)| i == j) {
                    for j in (j..j + 5).rev() {
                        println!("{} {}", i + 1, j + 1);
                    }
                    return;
                }
            }

            if i + 5 <= h {
                {
                    let mut bad = false;
                    for k in 0..5 {
                        if t[k] != s[i + k][j] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in 0..5 {
                            println!("{} {}", i + k + 1, j + 1);
                        }
                        return;
                    }
                }

                {
                    let mut bad = false;
                    for k in 0..5 {
                        if u[k] != s[i + k][j] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in (0..5).rev() {
                            println!("{} {}", i + k + 1, j + 1);
                        }
                        return;
                    }
                }
            }

            if i + 5 <= h && j + 5 <= w {
                {
                    let mut bad = false;
                    for k in 0..5 {
                        if t[k] != s[i + k][j + k] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in 0..5 {
                            println!("{} {}", i + k + 1, j + k + 1);
                        }
                        return;
                    }
                }

                {
                    let mut bad = false;
                    for k in 0..5 {
                        if u[k] != s[i + k][j + k] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in (0..5).rev() {
                            println!("{} {}", i + k + 1, j + k + 1);
                        }
                        return;
                    }
                }
            }

            if i + 5 <= h && j >= 4 {
                {
                    let mut bad = false;
                    for k in 0..5 {
                        if t[k] != s[i + k][j - k] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in 0..5 {
                            println!("{} {}", i + k + 1, j - k + 1);
                        }
                        return;
                    }
                }

                {
                    let mut bad = false;
                    for k in 0..5 {
                        if u[k] != s[i + k][j - k] {
                            bad = true;
                        }
                    }

                    if !bad {
                        for k in (0..5).rev() {
                            println!("{} {}", i + k + 1, j - k + 1);
                        }
                        return;
                    }
                }
            }
        }
    }
}
