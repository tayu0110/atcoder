use proconio::*;

fn main() {
    input! {n: usize, a: [u8; n]}

    let mut res = usize::MAX;
    for i in 0..n {
        for j in 1..=3 {
            let mut a = a.clone();
            a[i] = j;

            let mut stack: Vec<(u8, usize)> = vec![];
            for a in a {
                loop {
                    match stack.last_mut() {
                        Some(p) if p.0 == a => {
                            p.1 += 1;
                        }
                        Some(p) => {
                            if p.1 >= 4 {
                                stack.pop();
                                continue;
                            } else {
                                stack.push((a, 1));
                            }
                        }
                        None => {
                            stack.push((a, 1));
                        }
                    }

                    break;
                }
            }

            if stack.last().map_or(0, |s| s.1) >= 4 {
                stack.pop();
            }
            assert!(stack.iter().all(|s| s.1 < 4));

            res = res.min(stack.into_iter().map(|s| s.1).sum::<usize>());
        }
    }

    println!("{res}");
}
