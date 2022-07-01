use proconio::input;

fn split_and_list(n: usize, a: &[usize]) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; n+1];

    for i in 0..(1<<n) {
        let mut sum = 0;
        for (j, v) in a.iter().enumerate() {
            if (i & (1 << j)) != 0 {
                sum += *v;
            }
        }

        res[(i as i32).count_ones() as usize].push(sum);
    }

    res
}

fn main() {
    input! {n: usize, k: usize, p: usize, a: [usize; n]};

    let b = &a[(n+1)/2..n];
    let a = &a[0..(n+1)/2];

    let ares = split_and_list(a.len(), a);
    let mut bres = split_and_list(b.len(), b);

    let mut res = 0usize;
    for (i, a) in ares.iter().enumerate() {
        if i > k || k - i >= bres.len() {
            continue;
        }
        let j = k - i;
        bres[j].sort();
        for v in a {
            let mut l = -1;
            let mut r = bres[j].len() as i32;
            while r - l > 1 {
                let m = (r + l) / 2;
                if bres[j][m as usize] + *v <= p {
                    l = m;
                } else {
                    r = m;
                }
            }
            res += (l+1) as usize;
        }
    }

    println!("{}", res);
}