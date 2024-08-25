use proconio::*;

fn main() {
    input! {mut a: [usize; 3]}
    a.sort();

    let mut res = 0;
    while a[0] + 2 <= a[2] {
        a[0] += 2;
        res += 1;
    }
    while a[1] + 2 <= a[2] {
        a[1] += 2;
        res += 1;
    }

    if a[0] == a[1] && a[1] == a[2] {
        println!("{}", res)
    } else if a[0] == a[1] {
        println!("{}", res + 1)
    } else {
        println!("{}", res + 2)
    }
}
