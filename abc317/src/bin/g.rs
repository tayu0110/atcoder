use itertools::Itertools;
use proconio::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

fn main() {
    input! {n: usize, m: usize, mut a: [[usize; m]; n]}

    let mut rng = thread_rng();
    let clock = SystemTime::now();
    let mut valid = false;
    let mut cnt = vec![0; n];
    'base: loop {
        if clock.elapsed().unwrap().as_millis() > 1950 {
            break;
        }

        let mut used = vec![vec![false; n + 1]; m];
        for i in 0..m {
            used[i][a[0][i]] = true;
        }
        for i in 1..n {
            for j in 0..m {
                let now = a[i][j];
                if used[j][now] {
                    if j == m - 1 {
                        if cnt[i] > 10 && i > 1 {
                            cnt[i] = 0;
                            let tar = rng.gen_range(1..i);
                            a[tar..i].iter_mut().for_each(|v| v.shuffle(&mut rng));
                        }
                        a[i].shuffle(&mut rng);
                        cnt[i] += 1;
                        continue 'base;
                    }
                    for _ in 0..m {
                        let k = rng.gen_range(j + 1..m);
                        if a[i][k] != now {
                            a[i].swap(j, k);
                            break;
                        }
                    }
                }

                let now = a[i][j];
                if used[j][now] {
                    if cnt[i] > 10 && i > 1 {
                        cnt[i] = 0;
                        let tar = rng.gen_range(1..i);
                        a[tar..i].iter_mut().for_each(|v| v.shuffle(&mut rng));
                    }
                    a[i].shuffle(&mut rng);
                    cnt[i] += 1;
                    continue 'base;
                }

                used[j][now] = true;
            }
        }

        valid = true;
        break;
    }

    if valid {
        println!("Yes");
        for a in a {
            println!("{}", a.iter().join(" "))
        }
    } else {
        println!("No")
    }
}
