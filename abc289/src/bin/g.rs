use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut b: [usize; n], c: [usize; m]}
    b.sort();

    let b = {
        let mut buf = vec![];
        for (i, &b) in b.iter().enumerate() {
            buf.push(b * (n - i));
        }

        let mut res = vec![(b[0], 0)];
        for i in 1..n {
            if buf[i - 1] <= buf[i] {
                res.push((b[i], i));
            }
        }
        res
    };

    eprintln!("b: {:?}", b);
    // eprintln!("b: {:?}", b);

    for c in c {
        let (mut l, mut r) = (0, b.len());
        while r - l > 3 {
            let (m1, m2) = ((l * 2 + r) / 3, (l + r * 2) / 3);
            let res1 = {
                let (b, index) = b[m1];
                (b + c) * (n - index)
            };
            let res2 = {
                let (b, index) = b[m2];
                (b + c) * (n - index)
            };

            if res1 > res2 {
                r = m2;
            } else {
                l = m1;
            }
        }

        let mut max = 0;
        for i in l.saturating_sub(5)..std::cmp::min(b.len(), r.saturating_add(5)) {
            let (b, index) = b[i];
            max = std::cmp::max(max, (b + c) * (n - index));
        }

        println!("{}", max);
    }
    // for c in c {
    //     let mut max = 0;
    //     for (i, &b) in b.iter().enumerate() {
    //         max = std::cmp::max(max, (b + c) * (n - i));
    //     }

    //     println!("{}", max);
    // }
}
