use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{Arg, Command};

fn main() {
    let args = Command::new("read-from-file")
        .version("0.1")
        .author("Sangchual CHA <sangchual.cha@gmail.com>")
        .about("read lines from file written in Rust")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .required(true)
            .help("Read one or more newline separated patterns from file."))
        .get_matches();

    let _file_to_read = args.get_one::<String>("file").unwrap();
    let _fp = File::open(_file_to_read).unwrap();
    let mut _reader = BufReader::new(_fp);
    let mut _line = String::new();

    loop {
        let _len = _reader.read_line(&mut _line).unwrap();
        if _len == 0 {
            break;
        }
        println!("{} ({} bytes long)", _line, _len);

        _line.truncate(0);
    }
}

/*
➜  read-from-file git:(main) ✗ cargo run -- --file Cargo.toml
   Compiling read-from-file v0.1.0 (/Users/chasc/workspace/rust/read-from-file)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/read-from-file --file Cargo.toml`
[package]
 (10 bytes long)
name = "read-from-file"
 (24 bytes long)
version = "0.1.0"
 (18 bytes long)
edition = "2021"
 (17 bytes long)

 (1 bytes long)
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
 (97 bytes long)

 (1 bytes long)
[dependencies]
 (15 bytes long)
clap = "4" (10 bytes long)

 */
