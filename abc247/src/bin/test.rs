use proconio::input;

fn main() {
    input! {n: usize, x: usize, y: usize, a: [usize; n], res: usize}

    let mut correct = 0;
    for i in 0..n {
        for j in i+1..=n {
            if a[i..j].iter().any(|c| *c == x) && a[i..j].iter().any(|c| *c == y) && a[i..j].iter().all(|c| &y <= c && c <= &x) {
                correct += 1;
            }
        }
    }

    assert_eq!(res, correct);
}