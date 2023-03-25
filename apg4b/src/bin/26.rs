use std::collections::HashMap;

fn parse_int_expr(instr: &[&str], variables: &HashMap<String, i32>) -> i32 {
    let mut res = 0;
    let mut plus = 1;

    for ch in instr.iter().collect::<Vec<_>>().chunks_exact(2) {
        if let &[var, op] = ch {
            let var = if variables.contains_key(&var.to_string()) {
                *variables.get(&var.to_string()).unwrap()
            } else {
                var.parse().unwrap()
            };
    
            res += var * plus;
    
            if op == &"+" {
                plus = 1;
            } else if op == &"-" {
                plus = -1;
            }
        }
    }

    res
}

fn parse_vec_expr(instr: &[&str], variables: &HashMap<String, i32>, vectors: &HashMap<String, Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    let mut plus = 1;

    let mut now = 0;
    while now < instr.len() {
        if instr[now] == "[" {
            now += 1;
            let mut idx = 0;
            while instr[now] != "]" {
                if instr[now] == "," {
                    now += 1;
                    continue;
                }

                let var = if variables.contains_key(&instr[now].to_string()) {
                    *variables.get(&instr[now].to_string()).unwrap()
                } else {
                    instr[now].parse().unwrap()
                };

                if res.len() == idx {
                    res.push(var);
                } else {
                    res[idx] += var * plus;
                }

                idx += 1;
                now += 1;
            }
        } else if instr[now] == "+" {
            plus = 1;
        } else if instr[now] == "-" {
            plus = -1;
        } else if vectors.contains_key(&instr[now].to_string()) {
            let v = vectors.get(&instr[now].to_string()).unwrap();
            for (i, v) in v.iter().enumerate() {
                if i == res.len() {
                    res.push(*v);
                } else {
                    res[i] += *v * plus;
                }
            }
        }

        now += 1;
    }

    res
}

fn main() {
    let n: i32 = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut ws = buf.split_whitespace();
        ws.next().unwrap().parse().unwrap()
    };

    let mut variables = HashMap::new();
    let mut vectors = HashMap::new();

    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let instr = {
            let mut v = vec![];
            for nv in buf.split_whitespace() {
                v.push(nv);
            }
            v
        };

        match instr[0] {
            "int" => {
                let res = parse_int_expr(&instr[3..], &variables);
                variables.insert(instr[1].to_string(), res);
            }
            "print_int" => {
                let res = parse_int_expr(&instr[1..], &variables);
                println!("{}", res);
            }
            "vec" => {
                let res = parse_vec_expr(&instr[3..], &variables, &vectors);
                vectors.insert(instr[1].to_string(), res);
            }
            "print_vec" => {
                let res = parse_vec_expr(&instr[1..], &variables, &vectors);
                print!("[");
                for e in res {
                    print!(" {}", e);
                }
                println!(" ]");
            }
            _ => unreachable!("Why reach here?")
        }
    }
}