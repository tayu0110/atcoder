use proconio::*;

fn main() {
    input! {n: usize, mut a: [i32; n]}

    let mut used = vec![false; n + 1];
    for &a in &a {
        if a > 0 {
            used[a as usize] = true;
        }
    }

    let mut unused = used
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|u| (!u.1).then_some(u.0))
        .collect::<Vec<_>>();

    for a in a.iter_mut() {
        if *a < 0 {
            *a = unused.pop().unwrap() as i32;
        }
    }

    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            res = res.max(((i + 1) as i32 * a[i]).abs_diff(a[j] * (j + 1) as i32));
        }
    }

    println!("{res}")
}
