use proconio::*;

fn main() {
    input! {n: usize, mut k: usize, mut a: [usize; n]}
    a.sort();

    if a[0] > 1 {
        for i in 1..a[0] {
            println!("{}", i);
            k -= 1;
            if k == 0 {
                return;
            }
        }
    }

    let mut log = 0;
    let mut m = 0;
    for i in 0..=60 {
        if a.contains(&(1usize << i)) {
            let p = a.iter().position(|v| *v == 1 << i).unwrap();
            a.remove(p);
            continue;
        }

        log = i - 1;
        m = (1usize << i) - 1;
        break;
    }
}
