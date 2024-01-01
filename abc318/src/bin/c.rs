use proconio::*;

fn main() {
    input! {n: usize, d: usize, p: usize, mut f: [usize; n]}
    f.sort();

    let mut sum = f.iter().sum::<usize>();
    let mut res = sum;
    for now in 1.. {
        let mut cnt = 0;
        while let Some(f) = f.pop() {
            sum -= f;
            cnt += 1;
            if cnt >= d {
                break;
            }
        }

        res = res.min(sum + now * p);

        if f.is_empty() {
            break;
        }
    }

    println!("{res}")
}
