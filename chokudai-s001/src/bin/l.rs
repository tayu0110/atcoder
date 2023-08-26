use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.iter_mut().for_each(|v| *v -= 1);

    let mut cnt = 0;
    for i in 0..n {
        while a[i] != i {
            let j = a[i];
            a.swap(i, j);
            cnt += 1;
        }
    }

    if (n - cnt) % 2 == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}
