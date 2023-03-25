fn main() {
    const N: usize = 600851475143;
    let mut n = N;

    let mut res = 0;
    for i in (2..=n).take_while(|v| *v * *v <= N) {
        while n % i == 0 {
            res = std::cmp::max(res, i);
            n /= i;
        }
    }

    if n > 1 {
        res = std::cmp::max(res, n);
    }

    println!("{}", res);
}