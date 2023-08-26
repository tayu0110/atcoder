use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut res = 0;
    for v in a.windows(2) {
        if v[0] < v[1] {
            res += v[1] - v[0];
        } else {
            res += v[1] + 1;
        }
    }

    let mut cum = vec![0i32; m];
    for v in a.windows(2) {
        if v[0] < v[1] {
            cum[(v[0] + 1) % m] += 1;
            cum[v[1]] -= 1;
        } else {
            cum[(v[0] + 1) % m] += 1;
            if (v[0] + 1) % m != 0 {
                cum[0] += 1;
            }
            cum[v[1]] -= 1;
        }
    }

    for i in 0..m - 1 {
        cum[i + 1] += cum[i];
    }

    let mut r = vec![0; m];
    for v in a.windows(2) {
        r[v[1]] += (v[1] + m - v[0] - 1) % m;
    }

    let mut now = res;
    for i in 0..m {
        now += r[i];
        now -= cum[i] as usize;
        res = res.min(now);
    }

    println!("{}", res)
}
