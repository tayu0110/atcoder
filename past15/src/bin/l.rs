use proconio::*;

fn solve(a: &mut [Vec<i32>], b: &mut [Vec<i32>]) {
    let h = a.len();
    let w = a[0].len();
    let mut col = 0;
    for i in 0..h {
        'b: while col < w && a[i][col] < 0 {
            for j in i + 1..h {
                if a[j][col] >= 0 {
                    a.swap(i, j);
                    b.swap(i, j);
                    break 'b;
                }
            }
            col += 1;
        }

        if col == w {
            break;
        }

        for j in i..h {
            if a[j][col] < 0 {
                continue;
            }

            let t = a[j][col];
            for k in col..w {
                if a[j][k] >= 0 {
                    a[j][k] ^= t;
                }
            }
        }

        col += 1;
    }
}

fn main() {
    input! {h: usize, w: usize, mut a: [[i32; w]; h], mut b: [[i32; w]; h]}

    solve(&mut b, &mut a);
    solve(&mut a, &mut b);

    eprintln!("a: {a:?}, b: {b:?}");

    for j in 0..w {
        let mut buf = vec![];
        for i in 0..h {
            if b[i][j] >= 0 {
                buf.push(a[i][j] ^ b[i][j]);
            }
        }
        buf.sort_unstable();
        buf.dedup();

        if buf.len() > 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
