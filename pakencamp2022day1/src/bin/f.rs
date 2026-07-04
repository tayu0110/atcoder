use proconio::*;

fn main() {
    input! {n: usize, m: usize, d: usize, l: [usize; n], a: [[usize; m]; n]}

    let mut res = -1;
    for i in 1..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                sum += l[j];
            }
        }

        if sum <= d {
            let mut max = vec![0; m];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    for k in 0..m {
                        max[k] = max[k].max(a[j][k]);
                    }
                }
            }
            res = res.max(max.iter().sum::<usize>() as i64);
        }
    }

    println!("{res}")
}
