use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [i32; 26]}

    if let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z] = a[0..] {
        println!("{}", a - b);
        println!("{}", c + d);
        println!("{}", std::cmp::max(0, f-e+1));
        println!("{}", (g+h+i)/3+1);
        
        let dagabaji = "dagabaji".chars().collect_vec();
        let mut res = "zzzzzzzzzzzzzzzzzzz".to_string();
        for i in 0..1<<8 {
            if (i as i32).count_ones() == j as u32 {
                let mut s = String::new();
                for j in 0..8 {
                    if i & (1 << j) != 0 {
                        s.push(dagabaji[j]);
                    }
                }
                res = std::cmp::min(res, s);
            }
        }
        println!("{}", res);

        let mut like = 0;
        let mut res = vec![];
        for i in 0..1000000 {
            let num = 59 * i + k;
            if num % 61 == l {
                res.push(num);
                if res.len() == m as usize {
                    like = num;
                    break;
                }
            }
        }

        let perfect = [6, 28, 496, 8128, 3355033];
        for p in perfect.iter() {
            if (like - *p).abs() >= n {
                println!("{}", std::cmp::min(like, *p));
                println!("{}", std::cmp::max(like, *p));
                break;
            }
        }

        println!("{}", (o+p+q) * (r+s+t) % 9973 * (u+v+w) % 9973 * (x+y+z) % 9973);
    }
}
