use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m], s: [marker::Chars; n]}

    let mut t = vec![];
    for (i, s) in s.iter().enumerate() {
        let mut sum = 0;
        for i in 0..m {
            if s[i] == 'o' {
                sum += a[i];
            }
        }
        t.push(sum + i + 1);
    }

    for i in 0..n {
        let mut max = 0;
        for j in 0..n {
            if i != j {
                max = max.max(t[j]);
            }
        }

        if max < t[i] {
            println!("0");
            continue;
        }

        let mut rem = vec![];
        for j in 0..m {
            if s[i][j] == 'x' {
                rem.push(a[j]);
            }
        }

        let mut sum = t[i];
        rem.sort();
        let mut res = 0;
        while let Some(now) = rem.pop() {
            sum += now;
            res += 1;
            if sum > max {
                println!("{res}");
                break;
            }
        }
    }
}
