use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let cnt = std::thread::available_parallelism().unwrap().get();
    let mut th = vec![];
    for i in 0..cnt {
        let t = std::thread::spawn(move || {
            println!("Thread {i} is started!!!");

            let mut rng = thread_rng();

            let now = std::time::SystemTime::now();
            'B: while now.elapsed().unwrap().as_secs() < 60 {
                let mut t = vec![];
                for _ in 0..9 {
                    let mut buf = (1..=9).collect::<Vec<_>>();
                    buf.shuffle(&mut rng);
                    t.push(buf);
                }

                for j in 0..9 {
                    let mut f = [false; 9];
                    for i in 0..9 {
                        f[t[i][j] - 1] = true;
                    }

                    if f.iter().any(|f| !*f) {
                        continue 'B;
                    }
                }

                println!("Success!!!");
                println!("------------------------------");
                for v in t {
                    println!("{}", v.iter().join(""));
                }
                println!("------------------------------");

                return;
            }
        });

        th.push(t);
    }

    for th in th {
        th.join().ok();
    }
}
