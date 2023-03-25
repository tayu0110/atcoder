use proconio::input;

fn main() {
    input! {n: usize, l: [usize; n]}

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let mut v = vec![l[i], l[j], l[k]];
                v.sort();
                v.dedup();
                if v.len() != 3 {
                    continue;
                }

                if v[0] + v[1] > v[2] {
                    res += 1;
                }
            }
        }
    }

    println!("{}", res);
}
