use proconio::input;

fn check(a: usize) -> bool {
    for i in (2..=a).take_while(|v| *v * *v <= a) {
        if a % i == 0 {
            return true;
        }
    }
    false
}

fn main() {
    loop {
        input! {n: usize, a: [[usize; n]; n]}
    
        let mut ck = std::collections::HashSet::new();
        for i in 0..n {
            for j in 0..n {
                if ck.contains(&a[i][j]) {
                    println!("n: {}, i: {}, j: {}, check failed!!!!", n, i, j);
                    std::process::exit(0);
                }
                ck.insert(a[i][j]);
                if i > 0 {
                    let k = a[i][j] + a[i-1][j];
                    if !check(k) {
                        println!("n: {}, i: {}, j: {}, check failed!!!!", n, i, j);
                        std::process::exit(0);
                    }
                }
                if j > 0 {
                    let k = a[i][j] + a[i][j-1];
                    if !check(k) {
                        println!("n: {}, i: {}, j: {}, check failed!!!!", n, i, j);
                        std::process::exit(0);
                    }
                }
            }
        }
    }
}
