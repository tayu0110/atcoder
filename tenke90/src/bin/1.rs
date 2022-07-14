use proconio::input;

fn main() {
    input! {n: usize, l: usize, k: usize, a: [usize; n]};

    let mut a = a;
    a.push(l);

    let a = {
        let mut buf = vec![];
        for i in 0..n+1 {
            if i == 0 {
                buf.push(a[i]);
            } else {
                buf.push(a[i] - a[i-1]);
            }
        }
        buf
    };

    let mut r = l + 1;
    let mut l = 0;
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut len = 0;
        let mut cnt = 0;
        for v in &a {
            len += *v;
            if len >= m {
                len = 0;
                cnt += 1;
            }
        }

        if cnt >= k+1 {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}