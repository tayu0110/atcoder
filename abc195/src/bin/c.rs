use proconio::*;

fn main() {
    input! {n: usize}

    let d = n.to_string().len();
    let mut res = 0usize;
    for i in 4..=d {
        let c = (i - 1) / 3;
        let m = 10usize.pow(i as u32 - 1);
        let k = 10usize.pow(i as u32);
        if k > n {
            res += (n - m + 1) * c;
        } else {
            res += (k - m) * c;
        }
    }

    println!("{}", res)
}
