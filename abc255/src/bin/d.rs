use proconio::input;

fn main() {
    input! {n: usize, q: usize, a: [usize; n], x: [usize; q]};

    let mut a = a;
    a.sort();
    let a = a;

    let sum = a.iter().fold(0, |sum, x| sum + *x);
    let b = {
        let mut prev = -1;
        let mut buf: Vec<(usize, usize)> = vec![];
        for v in &a {
            if prev == *v as i64 {
                buf.last_mut().unwrap().1 += 1;
            } else {
                buf.push((*v, 1));
                prev = *v as i64;
            }
        }

        buf
    };

    let c = {
        let mut buf = vec![];
        let mut prev = 0;
        let mut sum = sum;
        let mut front = 0;
        let mut back = n;

        for (v, w) in &b {
            if prev == *v {
                buf.push((sum, front));
                front += *w;
                back -= *w;
            } else {
                sum = sum - back * (*v - prev) + front * (*v - prev);
                buf.push((sum, front));
                back -= *w;
                front += *w;
                prev = *v;
            }
        }

        buf
    };

    // eprintln!("{:?}", b);
    // eprintln!("{:?}", c);

    for v in x {
        let mut l = 0;
        let mut r = b.len();
        while r - l > 1 {
            let m = (r + l) / 2;
            let (now, _) = b[m];
            if now < v {
                l = m;
            } else {
                r = m;
            }
        }

        let (now, num) = b[l];
        let (sum, front) = c[l];
        if now == v {
            println!("{}", sum);
        } else if now > v {
            println!("{}", sum + (now - v) * n);
        } else {
            println!("{}", sum - (n - front - num) * (v - now) + (front + num) * (v - now));
        }
    }

}