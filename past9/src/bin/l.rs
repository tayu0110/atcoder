use proconio::*;

fn main() {
    input! {n: usize, p: usize, mut a: [i32; n]}

    a.reverse();
    a.iter_mut().enumerate().for_each(|(i, a)| *a -= i as i32);

    let mut lis = vec![i32::MAX; n];
    for a in a {
        if 0 <= a && a as usize <= p + 1 - n {
            let pos = lis.partition_point(|&l| l <= a);
            lis[pos] = lis[pos].min(a);
        }
    }

    println!("{}", n - lis.iter().take_while(|&&l| l < i32::MAX).count());
}
