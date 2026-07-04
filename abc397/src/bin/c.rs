use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ka = vec![0; n + 1];
    let mut kb = vec![0; n + 1];
    for i in 0..n {
        ka[a[i]] += 1;
    }

    let mut res = 0;
    let mut s = ka.iter().filter(|&&ka| ka > 0).count();
    let mut t = 0;
    for i in 0..n {
        ka[a[i]] -= 1;
        if ka[a[i]] == 0 {
            s -= 1;
        }
        kb[a[i]] += 1;
        if kb[a[i]] == 1 {
            t += 1;
        }

        res = res.max(s + t);
    }

    println!("{res}")
}
