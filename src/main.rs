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
