use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ka = vec![0; n + 1];
    for i in 0..n {
        ka[a[i]] += 1;
    }

    let mut kb = vec![0; n + 1];
    let mut max = 0;
    let mut now = ka.iter().filter(|&&ka| ka > 0).count();
    let mut sep = 0;
    for i in 0..n {
        ka[a[i]] -= 1;
        kb[a[i]] += 1;
        if ka[a[i]] == 0 {
            now -= 1;
        }
        if kb[a[i]] == 1 {
            now += 1;
        }

        if max < now {
            sep = i + 1;
            max = now;
        }
    }

    let mut ka = vec![0; n + 1];
    let mut kb = vec![0; n + 1];
    for i in 0..sep {
        ka[a[i]] += 1;
    }
    for i in sep..n {
        kb[a[i]] += 1;
    }

    let mut max = vec![0; n];
    max[n - 1] = ka.iter().filter(|&&a| a > 0).count() + kb.iter().filter(|&&b| b > 0).count();
    let mut good = vec![0; n];
    good[n - 1] = sep;
    for i in (1..n).rev() {
        let mut sep = good[i];
        let mut m = max[i];
        kb[a[i]] -= 1;
        if kb[a[i]] == 0 {
            m -= 1;
        }
        while (sep > 0 && (kb[a[sep - 1]] == 0 || ka[a[sep - 1]] > 1)) || sep == i {
            kb[a[sep - 1]] += 1;
            ka[a[sep - 1]] -= 1;
            if kb[a[sep - 1]] == 1 {
                m += 1;
            }
            if ka[a[sep - 1]] == 0 {
                m -= 1;
            }
            sep -= 1;
        }
        good[i - 1] = sep;
        max[i - 1] = m;
    }

    let mut kind = vec![0; n + 1];
    let mut count = 0;
    for i in 1..n {
        kind[a[i]] += 1;
        if kind[a[i]] == 1 {
            count += 1;
        }
    }

    let mut res = 0;
    for i in 1..n - 1 {
        kind[a[i]] -= 1;
        if kind[a[i]] == 0 {
            count -= 1;
        }
        res = res.max(count + max[i]);
    }

    println!("{res}")
}
