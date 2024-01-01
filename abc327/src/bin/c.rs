use proconio::*;

fn main() {
    input! {a: [[usize; 9]; 9]}

    for i in 0..9 {
        let mut v = [false; 9];
        for j in 0..9 {
            v[a[i][j] - 1] = true;
        }

        if v.iter().any(|v| !*v) {
            println!("No");
            return;
        }
    }

    for j in 0..9 {
        let mut v = [false; 9];
        for i in 0..9 {
            v[a[i][j] - 1] = true;
        }

        if v.iter().any(|v| !*v) {
            println!("No");
            return;
        }
    }

    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut v = [false; 9];
            for k in i..i + 3 {
                for l in j..j + 3 {
                    v[a[k][l] - 1] = true;
                }
            }

            if v.iter().any(|v| !*v) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes")
}
