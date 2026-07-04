use proconio::*;

fn solve(t: bool, mut a: Vec<u32>, b: &[u32]) -> bool {
    let n = a.len();
    for i in 0..n {
        if a[i] != b[i] {
            if i + 1 == n {
                return false;
            }

            a.swap(i, i + 1);
            for j in i..n {
                if a[j] != b[j] {
                    if j + 1 == n {
                        return false;
                    }
                    a.swap(j, j + 1);
                    return a[j..] == b[j..];
                }
            }

            return t || a.windows(2).any(|v| v[0] == v[1]);
        }
    }

    true
}

fn main() {
    input! {n: usize, mut a: [u32; n], mut b: [u32; n]}

    let t = a.windows(2).any(|v| v[0] == v[1]);
    let f = solve(t, a.clone(), &b);
    if f {
        println!("Yes");
        return;
    }

    a.reverse();
    b.reverse();
    let f = solve(t, a, &b);
    if f {
        println!("Yes");
    } else {
        println!("No")
    }
}
