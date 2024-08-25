use num::BigUint;
use num_bigint::RandBigInt;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    'b: loop {
        let x = rng.gen_biguint(30000);

        let mut y = x.clone();
        let mut p = y
            .to_string()
            .chars()
            .map(|c| c as usize - b'0' as usize)
            .sum::<usize>();
        for _ in 0..50 {
            let z = y.clone() * BigUint::from(2u64);
            let q = z
                .to_string()
                .chars()
                .map(|c| c as usize - b'0' as usize)
                .sum::<usize>();
            if p <= q {
                continue 'b;
            }
            y = z;
            p = q;
        }

        println!("{x}");
        break;
    }
}
