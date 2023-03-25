use proconio::input;

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h], r: usize, c: usize, b: [[usize; c]; r]}

    for i in 0u32..(1 << h) {
        if i.count_ones() != r as u32 {
            continue;
        }

        let mut v = vec![];
        for k in 0..h {
            if i & (1 << k) != 0 {
                v.push(a[k].clone());
            }
        }
        'mid: for j in 0u32..(1 << w) {
            if j.count_ones() != c as u32 {
                continue;
            }

            let mut buf = vec![];
            for k in 0..w {
                if j & (1 << k) != 0 {
                    buf.push(k);
                }
            }

            for i in 0..r {
                for (j, &k) in buf.iter().enumerate() {
                    if b[i][j] != v[i][k] {
                        continue 'mid;
                    }
                }
            }

            println!("Yes");
            return;
        }
    }

    println!("No");
}
