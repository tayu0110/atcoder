use proconio::*;

fn solve(n: usize, a: Vec<usize>) -> usize {
    let mut res = 0;
    let mut ten = 1;
    for _ in 0..15 {
        ten *= 10;

        let mut t = a.iter().map(|&a| a % ten).collect::<Vec<_>>();
        t.sort();

        for &now in &t {
            let (mut l, mut r) = (-1, n as i32);
            while r - l > 1 {
                let m = (r + l) / 2;

                if now + t[m as usize] >= ten {
                    r = m;
                } else {
                    l = m;
                }
            }

            res += n - r as usize;
        }
    }

    res
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut buf = vec![vec![0; 10]; 15];
    for &a in &a {
        let mut now = a;
        for i in 0..15 {
            let z = now % 10;

            for j in 0..10 {
                res += (j + z) * buf[i][j] * 2;
            }

            buf[i][z] += 1;
            now /= 10;

            res += z * 2;
        }
    }

    let k = solve(n, a);

    res -= k * 9;
    println!("{}", res);
}
