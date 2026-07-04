use itertools::Itertools;
use proconio::*;
// use rand::{thread_rng, Rng};

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    // let mut max = 0;
    // for _ in 0..1000000 {
    //     let mut rng = thread_rng();
    //     let n = 50;
    //     // let k: usize = rng.gen_range(1..=50);
    //     let k = 1usize;
    //     let mut a = (0..n).map(|_| rng.gen_range(1..=50)).collect::<Vec<_>>();

    if n == 2 {
        if a[0] <= a[1] {
            println!("Yes");
            println!("0")
        } else {
            let mut res = vec![];
            while a[0] > a[1] && res.len() < 500000 {
                a.swap(0, 1);
                a[0] += k;
                res.push(1);
            }
            if res.len() <= 500000 && a[0] <= a[1] {
                println!("Yes");
                println!("{}", res.len());
                println!("{}", res.iter().join(" "));
            } else {
                println!("No")
            }
        }
        return;
    }

    let mut res = vec![];
    while a.windows(2).any(|v| v[0] > v[1]) {
        let mut i = 0;
        while i < n - 1 {
            if a[i] <= a[i + 1] {
                i += 1;
                continue;
            }

            if a[i] >= a[i + 1] + k {
                res.push(i + 1);
                a.swap(i, i + 1);
                a[i] += k;
                i += 1;
                continue;
            }

            if i + 2 < n {
                while a[i] > a[i + 1] {
                    res.push(i + 2);
                    a.swap(i + 1, i + 2);
                    a[i + 1] += k;
                }
                i += 1;
            } else {
                i -= 1;
                while a[i] + k >= a[i + 1] || a[i] + k >= a[i + 2] {
                    res.push(i + 2);
                    a.swap(i + 1, i + 2);
                    a[i + 1] += k;
                }
                res.push(i + 1);
                a.swap(i, i + 1);
                a[i] += k;
                while a[i] > a[i + 1] || a[i] > a[i + 2] || a[i + 1] > a[i + 2] {
                    res.push(i + 2);
                    a.swap(i + 1, i + 2);
                    a[i + 1] += k;
                }
                break;
            }
        }
    }

    assert!(a.windows(2).all(|v| v[0] <= v[1]), "{a:?}");
    assert!(res.len() <= 500000);
    println!("Yes");
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
    // max = max.max(res.len());
    // }
    // println!("max: {max}");
}
