use proconio::input;

fn main() {
    input! {t: usize}

    'base: for _ in 0..t {
        input! {n: usize, a: [usize; n], mut b: [usize; n]}

        if a == b {
            println!("Yes");
            continue;
        }

        if b.windows(2).all(|v| v[0] != v[1]) && b[0] != b[n - 1] {
            println!("No");
            continue;
        }

        'mid: for i in 0..n {
            let c = [&b[i..n], &b[0..i]].concat();

            let mut j = 0;
            for c in c {
                while j < n && a[j] != c {
                    j += 1;
                }

                if j == n {
                    continue 'mid;
                }
            }

            println!("Yes");
            continue 'base;
        }

        println!("No");
    }
}
