extern crate peg;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::io::{stdin, stdout, stderr};
use std::path::Path;
use std::process;

fn main() {
	let args = env::args_os().collect::<Vec<_>>();
	let progname = &args[0];
	let mut log = stderr();

	let mut source = String::new();

	if args.len() == 2 && &args[1] != "-h" {
		let fname = Path::new(&args[1]);
		File::open(fname).unwrap().read_to_string(&mut source).unwrap();
	} else if args.len() == 1 {
		stdin().read_to_string(&mut source).unwrap();
	} else {
		writeln!(log, "Usage: {} [file]", progname.to_string_lossy()).unwrap();
		process::exit(0);
	}

	match peg::compile(&source) {
		Ok(parser) => {
			let mut out = stdout();
			writeln!(&mut out, "// Generated by rust-peg. Do not edit.").unwrap();
			write!(&mut out, "{}", parser).unwrap();
		}
		Err(err) => {
			writeln!(log, "Error compiling rust-peg grammar:").unwrap();
			writeln!(log, "{}", err).unwrap();
			process::exit(1);
		}
	}
}
