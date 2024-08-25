use proconio::*;

fn main() {
    input! {n: usize, q: [usize; n], a: [usize; n], b: [usize; n]}

    let mut res = 0;
    for i in 0.. {
        let mut q = q.clone();
        for j in 0..n {
            if q[j] < a[j] * i {
                println!("{res}");
                return;
            }

            q[j] -= a[j] * i;
        }

        let mut min = usize::MAX;
        for j in 0..n {
            if b[j] > 0 {
                min = min.min(q[j] / b[j]);
            }
        }

        res = res.max(i + min);
    }
}
