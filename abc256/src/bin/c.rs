use proconio::input;

fn main() {
    input! {a: usize, b: usize, c: usize, s: usize, t: usize, u: usize};

    let mut res = 0;
    for i in 1..35 {
        for j in 1..35 {
            for k in 1..35 {
                for l in 1..35 {
                    if i + j >= a || k + l >= b || i + k >= s || j + l >= t {
                        continue;
                    }
                    let m = a - i - j;
                    let n = b - k - l;
                    let o = s - i - k;
                    let p = t - j - l;
                    if m + n >= u {
                        continue;
                    }
                    let q = u - m - n;
                    if o + p + q == c {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);
}