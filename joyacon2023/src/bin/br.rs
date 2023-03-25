use proconio::input;

fn main() {
    input! {n: usize, mut a: [usize; n], q: usize}

    let mut base = 0;
    let mut updated = vec![-1; n];
    let mut cleared = -1;
    for i in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize}
            cleared = i as i32;
            base = x;
        } else if t == 2 {
            input! {j: usize, x: usize}
            if updated[j - 1] < cleared {
                a[j - 1] = 0;
            }
            updated[j - 1] = i as i32;
            a[j - 1] += x;
        } else {
            input! {j: usize}
            if updated[j - 1] < cleared {
                a[j - 1] = 0;
                updated[j - 1] = i as i32;
            }

            println!("{}", base + a[j - 1]);
        }
    }
}
