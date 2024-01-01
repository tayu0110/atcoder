use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: usize, mut b: usize, x: [usize; m]}

    let mut rem = vec![0; n];
    for x in x {
        rem[x - 1] += 1;
    }

    rem.sort_unstable();

    let mut l = 0;
    let mut diff = a - b;
    for r in rem {
        if r <= b {
            b -= r;
            continue;
        }

        if diff > 0 {
            l += 1;
            diff -= 1;
        }
    }

    println!("{}", n - l);
}
