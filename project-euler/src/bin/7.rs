fn main() {
    let mut primes = vec![2];

    for now in 3..1000000000 {
        let mut bad = false;
        for v in &primes {
            if now % *v == 0 {
                bad = true;
                break;
            }
        }

        if !bad {
            primes.push(now);
            if primes.len() == 10001 {
                break;
            }
        }
    }

    println!("{}", primes.last().unwrap());
}