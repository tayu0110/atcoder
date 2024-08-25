use proconio::input;

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]};

    let mut u = vec![];
    for j in 0..w {
        let mut sum = 0;
        for i in 0..h {
            sum += a[i][j];
        }
        u.push(sum);
    }

    let mut v = vec![];
    for w in &a {
        let sum = w.iter().fold(0, |sum, x| sum + *x);
        v.push(sum);
    }

    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" ");
            }
            print!("{}", v[i] + u[j] - a[i][j]);
        }
        println!();
    }
}