use proconio::*;

fn main() {
    input! {n: usize, mut a: [i64; n]}

    let sum: i64 = a.iter().sum();
    let ave = sum.div_euclid(n as i64);

    let mut res = 0;
    for i in 0..n {
        if a[i] < ave {
            for j in i + 1..n {
                if a[j] > ave + 1 {
                    let diff = (a[j] - ave - 1).min(ave - a[i]);
                    a[i] += diff;
                    a[j] -= diff;
                    res += (j - i) * diff as usize;
                }

                if a[i] == ave {
                    break;
                }
            }
        } else if a[i] > ave + 1 {
            for j in i + 1..n {
                if a[j] < ave {
                    let diff = (ave - a[j]).min(a[i] - ave - 1);
                    a[i] -= diff;
                    a[j] += diff;
                    res += (j - i) * diff as usize;
                }

                if a[i] == ave + 1 {
                    break;
                }
            }
        }

        // eprintln!("res: {res}");
    }

    // eprintln!("res: {res}, ave: {ave}");

    while !a.iter().all(|v| (*v - ave).abs() <= 1) {
        for i in (0..n).rev() {
            if a[i] < ave {
                for j in (0..i).rev() {
                    if a[j] > ave {
                        a[i] += 1;
                        a[j] -= 1;
                        res += i - j;
                    }

                    if a[i] == ave {
                        break;
                    }
                }
            } else if a[i] > ave + 1 {
                for j in (0..i).rev() {
                    if a[j] <= ave {
                        a[j] += 1;
                        a[i] -= 1;
                        // eprintln!("res: {res}");
                        res += i - j;
                    }

                    if a[i] == ave + 1 {
                        break;
                    }
                }
            } else if a[i] == ave + 1 && a.iter().any(|a| *a < ave) {
                for j in (0..i).rev() {
                    if a[j] < ave {
                        a[j] += 1;
                        a[i] -= 1;
                        // eprintln!("res: {res}");
                        res += i - j;
                    }

                    if a[i] == ave {
                        break;
                    }
                }
            }
        }

        // eprintln!("{a:?}");
    }

    println!("{}", res);
}
