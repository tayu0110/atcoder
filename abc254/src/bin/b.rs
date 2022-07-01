use proconio::input;

fn main() {
    input! {n: usize};

    let mut res = vec![];
    for i in 0..n {
        if res.is_empty() {
            println!("1");
            res.push(1);
        } else {
            let mut tmp = vec![];
            for j in 0..i {
                if j == 0 {
                    print!("1");
                    tmp.push(1);
                } else {
                    print!(" {}", res[j-1] + res[j]);
                    tmp.push(res[j-1] + res[j]);
                }
            }
            println!(" 1");
            tmp.push(1);
            std::mem::swap(&mut res, &mut tmp);
        }
    }
}