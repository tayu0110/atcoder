use proconio::*;

fn main() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: usize, d: usize, mut a: [usize; n], b: [usize; n]}

        let mut ret = 0;
        let mut cur = 0;
        for (i, mut b) in b.into_iter().enumerate() {
            ret += a[i];

            while cur <= i && b > 0 {
                if a[cur] > b {
                    a[cur] -= b;
                    ret -= b;
                    b = 0;
                } else {
                    b -= a[cur];
                    ret -= a[cur];
                    a[cur] = 0;
                    cur += 1;
                }
            }

            if i >= d {
                while cur <= i - d {
                    ret -= a[cur];
                    cur += 1;
                }
            }
        }

        println!("{ret}")
    }
}
