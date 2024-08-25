use proconio::*;

fn main() {
    input! {h: usize, _: usize, k: usize, mut s: [marker::Bytes; h]}

    let mut res = usize::MAX;
    for _ in 0..2 {
        if s[0].len() >= k {
            let n = s[0].len();
            for v in &s {
                let (mut l, mut r) = (0, 0);
                let mut sum = 0;
                let mut bad = 0;
                while l < n {
                    while r < n && r - l < k {
                        match v[r] {
                            b'o' => {}
                            b'x' => {
                                bad += 1;
                            }
                            b'.' => {
                                sum += 1;
                            }
                            _ => {}
                        }
                        r += 1
                    }

                    if bad == 0 && r - l == k {
                        res = res.min(sum);
                    }
                    match v[l] {
                        b'x' => bad -= 1,
                        b'.' => sum -= 1,
                        _ => {}
                    }
                    l += 1;
                }
            }
        }

        let (nh, nw) = (s[0].len(), s.len());
        let mut new = vec![vec![0; nw]; nh];
        for i in 0..s.len() {
            for j in 0..s[0].len() {
                new[j][i] = s[i][j];
            }
        }

        s = new;
    }

    println!("{}", res as i64)
}
