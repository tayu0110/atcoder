use proconio::*;

fn main() {
    input! {n: usize, q: usize, a: [usize; n]}

    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    let mut cum1 = vec![0; n + 1];
    for i in 0..n {
        cum1[i + 1] = cum1[i] + a[i] * (i + 1);
    }
    let mut cum2 = vec![0; n + 1];
    for i in 0..n {
        cum2[i + 1] = cum2[i] + a[i] * (i + 1) * (i + 1);
    }

    for _ in 0..q {
        input! {l: usize, r: usize}

        println!(
            "{}",
            (cum1[r] - cum1[l - 1]) * (l + r)
                - (cum2[r] - cum2[l - 1])
                - (l - 1) * (r + 1) * (cum[r] - cum[l - 1])
        );
    }
}
