use std::{fs::File, io::Write};

fn main() {
    let fname = format!("{}/test.sh", env!("CARGO_MANIFEST_DIR"));
    let mut file = File::create(fname).unwrap();

    file.write_all(b"#!/usr/bin/bash\ncargo build --release\ncargo build\n")
        .unwrap();
    for i in 0..100 {
        file.write_all(format!("echo '=============== case {i} ==============='\n").as_bytes())
            .unwrap();
        file.write_all(
            format!(
                "tools/target/debug/tester target/release/ahc22 < tools/in/{i:04}.txt > tools/out/{i:04}.txt\n"
            )
            .as_bytes(),
        )
        .unwrap();
        file.write_all("echo '======================================='\n".to_string().as_bytes())
            .unwrap();
    }
}
