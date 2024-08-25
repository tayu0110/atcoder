use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut n: usize, a: usize, m: usize}

        let mut used = vec![false; m];
        let mut perm = vec![];
        let mut now = 0;
        let mut na = 0;
        while {
            na += a;
            if now < na {
                now += (na - now + m - 1) / m * m;
            }
            !used[now - na]
        } {
            used[now - na] = true;
            perm.push((now - na) as u32);
        }

        if perm.len() >= n {
            println!("{}", perm[..n].iter().sum::<u32>());
            continue;
        }

        let pos = perm.iter().position(|&p| p as usize == now - na).unwrap();
        let mut res = perm[..pos].iter().sum::<u32>();
        n -= pos;
        res += perm[pos..].iter().sum::<u32>() * (n / (perm.len() - pos)) as u32;
        n %= perm.len() - pos;
        res += perm[pos..pos + n].iter().sum::<u32>();

        println!("{res}")
    }
}
