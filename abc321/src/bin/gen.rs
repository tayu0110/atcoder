use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    const Q: usize = 10;
    const K: usize = 10;
    const MAX: usize = 12;

    let mut query = vec![('+', rng.gen_range(1..=MAX))];
    let mut stack = vec![query[0].1];
    while query.len() < Q {
        if stack.is_empty() {
            query.push(('+', rng.gen_range(1..=MAX)));
        } else {
            let ty = rng.gen_bool(0.5);
            if ty {
                query.push(('+', rng.gen_range(1..=MAX)));
            } else {
                let t = stack.remove(rng.gen_range(0..stack.len()));
                query.push(('-', t));
            }
        }
    }

    println!("{Q} {K}");
    for (ty, x) in query {
        println!("{ty} {x}");
    }
}
