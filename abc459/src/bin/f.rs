use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, a: [i64; n]}

        let mut ret = i64::MAX;
        let sum = a.iter().sum::<i64>();
        let ave = sum / n as i64;
        for ave in ave - 5..ave + 5 {
            let mut b = vec![0; n];
            b[n / 2] = ave;
            for (i, c) in (0..n / 2).rev().zip((ave - n as i64 / 2..ave).rev()) {
                b[i] = c;
            }
            for (i, c) in (n / 2 + 1..n).zip(ave + 1..) {
                b[i] = c;
            }

            let s = b.iter().sum::<i64>();
            let diff = sum - s;
            if diff.abs() >= n as i64 {
                eprintln!("diff: {diff}");
                continue;
            }
            if diff > 0 {
                for i in (0..n).rev().take(diff as usize) {
                    b[i] += 1;
                }
            } else {
                for i in (0..n).take(diff.abs() as usize) {
                    b[i] -= 1;
                }
            }

            eprintln!("b: {b:?}");
            let mut tmp = 0;
            let mut a = a.clone();
            for i in 0..n - 1 {
                if a[i] < b[i] {
                    tmp = i64::MAX;
                    break;
                }

                let diff = a[i] - b[i];
                a[i] -= diff;
                a[i + 1] += diff;
                tmp += diff;
            }

            eprintln!("a: {a:?}");
            ret = ret.min(tmp);
        }
        println!("{}", ret);
    }
}
