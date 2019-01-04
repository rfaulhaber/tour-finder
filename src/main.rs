mod point;
use std::collections::HashMap;
use std::env;
use std::format;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        let open_err_message = format!("could not open file: {}", filename);
        let f = File::open(filename).expect(&open_err_message);
    } else {
        let mut buf = String::new();
        from_stdin(&mut buf).expect("failed to read from stdin");

        let points_str = buf.split("\n").collect::<Vec<&str>>();

        println!("from buf: {:?}", points_str);
    }
}

fn from_stdin(mut buffer: &mut String) -> io::Result<()> {
    io::stdin().read_to_string(&mut buffer)?;
    Ok(())
}
