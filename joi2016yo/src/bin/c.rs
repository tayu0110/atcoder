use proconio::*;

fn main() {
    input! {n: usize, _: usize, s: [marker::Bytes; n]}

    let mut res = usize::MAX;
    let mut w = 0;
    for i in 0..n {
        w += s[i].iter().filter(|&&s| s != b'W').count();
        let mut b = 0;
        for j in i + 1..n - 1 {
            b += s[j].iter().filter(|&&s| s != b'B').count();
            let mut r = 0;
            for k in j + 1..n {
                r += s[k].iter().filter(|&&s| s != b'R').count();
            }
            res = res.min(w + b + r);
        }
    }

    println!("{res}")
}
