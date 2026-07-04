use proconio::*;

fn make_doubling_table(next: Vec<usize>) -> Vec<Vec<usize>> {
    let n = next.len();
    let mut doubling = vec![next];
    for i in 0..20 {
        let mut next = vec![usize::MAX; n];
        for j in 0..n {
            if doubling[i][j] < n {
                next[j] = doubling[i][doubling[i][j]];
            }
        }
        doubling.push(next);
    }
    doubling
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let b = a.repeat(2);
    let mut cum = vec![0; b.len() + 1];
    for i in 0..b.len() {
        cum[i + 1] = cum[i] + b[i];
    }
    let (mut l, mut r) = (1, 2000_100_100);
    'b: while r - l > 1 {
        let m = (r + l) / 2;

        let mut next = vec![usize::MAX; b.len()];
        for i in 0..b.len() {
            let pos = cum.partition_point(|&c| c < m + cum[i]);
            if pos > i + n {
                r = m;
                continue 'b;
            }

            next[i] = pos;
        }

        let p = next[0];
        let doubling = make_doubling_table(next);

        for start in 0..p {
            let mut rem = k;
            let mut now = start;
            for i in (0..doubling.len()).rev() {
                if rem & (1 << i) != 0 && now < b.len() {
                    rem -= 1 << i;
                    now = doubling[i][now];
                }
            }

            if now <= start + n {
                l = m;
                continue 'b;
            }
        }

        r = m;
    }

    let mut next = vec![usize::MAX; b.len()];
    for i in 0..b.len() {
        let pos = cum.partition_point(|&c| c < l + cum[i]);
        next[i] = pos;
    }

    let doubling = make_doubling_table(next);

    let mut res = 0;
    for start in 0..n {
        let mut rem = k;
        let mut now = start;
        for i in (0..doubling.len()).rev() {
            if rem & (1 << i) != 0 && now < b.len() {
                rem -= 1 << i;
                now = doubling[i][now];
            }
        }

        if now > start + n {
            res += 1;
        }
    }

    println!("{l} {res}");
}
