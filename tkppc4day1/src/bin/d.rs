use proconio::*;

fn main() {
    input! {mut n: usize, mut a: [i64; n]}

    a.dedup();
    n = a.len();
    if n == 1 {
        println!("0");
        return;
    }

    if a[0] < a[1] {
        a.insert(0, std::i64::MAX);
        n += 1;
    } else {
        a.insert(0, std::i64::MIN);
        n += 1;
    }

    if a[n - 2] > a[n - 1] {
        a.push(std::i64::MAX);
    } else {
        a.push(std::i64::MIN);
    }

    let mut cnt = 0;
    for v in a.windows(3) {
        if v[0] > v[1] && v[1] < v[2] {
            cnt += 1;
        } else if v[0] < v[1] && v[1] > v[2] {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
