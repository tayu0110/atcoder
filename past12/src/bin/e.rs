use proconio::*;

fn main() {
    input! {r: usize, n: usize, m: usize, l: usize, s: [usize; l]}

    let mut pt = 0;
    for _ in 0..r {
        if pt == l {
            println!("No");
            return;
        }
        let mut now = 0;
        let mut rem = m;
        while rem > 0 && pt < l && now + s[pt] <= n {
            now += s[pt];
            pt += 1;
            rem -= 1;
            if now == n {
                break;
            }
        }

        if rem > 0 && now != n {
            println!("No");
            return;
        }
    }

    if pt != l {
        println!("No");
        return;
    }

    println!("Yes")
}
