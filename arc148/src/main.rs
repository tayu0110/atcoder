use proconio::input;

fn main() {
    const A: u64 = 524798123;
    const B: u64 = 543537843;
    const S: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    input! {n: usize}

    let mut map = std::collections::HashMap::new();
    map.insert("A".to_string(), A);
    map.insert("B".to_string(), B);

    for _ in 0..n {
        input! {op: String}

        if op == "add" {
            input! {dest: String, op1: String, op2: String}
            let op1 = if op1.len() == 1 && S.contains(&op1) {
                *map.get(&op1).unwrap()
            } else {
                op1.parse::<u64>().unwrap()
            };
            let op2 = if op2.len() == 1 && S.contains(&op2) {
                *map.get(&op2).unwrap()
            } else {
                op2.parse::<u64>().unwrap()
            };

            if dest.len() != 1 || !S.contains(&dest) {
                panic!()
            }

            map.insert(dest, op1.wrapping_add(op2));
        } else if op == "mul" {
            input! {dest: String, op1: String, op2: String}
            let op1 = if op1.len() == 1 && S.contains(&op1) {
                *map.get(&op1).unwrap()
            } else {
                op1.parse::<u64>().unwrap()
            };
            let op2 = if op2.len() == 1 && S.contains(&op2) {
                *map.get(&op2).unwrap()
            } else {
                op2.parse::<u64>().unwrap()
            };

            if dest.len() != 1 || !S.contains(&dest) {
                panic!()
            }

            map.insert(dest, op1.wrapping_mul(op2));
        } else if op == "rem" {
            input! {dest: String, op1: String}
            let op1 = if op1.len() == 1 && S.contains(&op1) {
                *map.get(&op1).unwrap()
            } else {
                op1.parse::<u64>().unwrap()
            };

            if dest.len() != 1 || !S.contains(&dest) {
                panic!()
            }

            map.insert(dest, op1 % 998244353);
        } else {
            unreachable!();
        }
    }

    eprintln!("map: {:?}", map);
    assert_eq!(*map.get("C").unwrap(), A * B % 1000_000_007);
}
