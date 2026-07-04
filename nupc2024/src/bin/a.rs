use proconio::*;

fn main() {
    input! {n: usize, mut a: [u32; n]}

    let sum = a.iter().cloned().map(|a| a as usize).sum::<usize>();
    if sum % n != 0 {
        println!("-1");
        return;
    }
    let ave = (sum / n) as u32;
    let mut stack = vec![];
    for i in 0..n {
        if a[i] > ave {
            stack.push((i, a[i] - ave));
        }
    }

    let mut res = 0;
    let mut stack = &mut stack[..];
    for i in (0..n).rev() {
        while a[i] < ave {
            let (j, over) = stack.last_mut().unwrap();
            if i < *j {
                println!("-1");
                return;
            }
            if *over > ave - a[i] {
                res += (i - *j) * (ave - a[i]) as usize;
                *over -= ave - a[i];
                break;
            } else {
                res += (i - *j) * *over as usize;
                a[i] += *over;
                let len = stack.len();
                stack = &mut stack[..len - 1];
            }
        }
    }

    println!("{res}")
}
