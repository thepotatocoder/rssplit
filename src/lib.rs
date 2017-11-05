
extern crate clap;
extern crate grep;
#[macro_use]
extern crate lazy_static;

pub mod message;
pub mod args;
pub mod pattern;

use std::fs::File;
use std::io::Read;
use std::string::String;

pub fn uumain (matches: clap::ArgMatches) -> String {
	//Is there a way to check which came first in clap?
	//Csplit adresses each of these flags by order of arrival

	if matches.occurrences_of("VERSION") != 0 {
		return message::VERSION.to_string();
	} else if matches.occurrences_of("HELP") != 0 {
		return message::USAGE.to_string();
	} else if matches.occurrences_of("INPUT") == 0 {
		return format!("{}: missing operand\n{}", message::NAME, message::TRYHELP.to_string());
	}
	
	let res = args::Args::from_matches(&matches);
	if res.1 != "" {
		return res.1;
	}

	let args = res.0;

	if (matches.occurrences_of("INPUT") == 1) && (matches.occurrences_of("FILE_PATTERN") == 0) {
		return format!("{}: missing operand after '{}'\n{}", message::NAME, args.input_file, message::TRYHELP.to_string());
	}

	let mut file = match File::open(&args.input_file) {
		Ok(f) => f,
		Err(e) => return format!("{}: cannot open '{}' for reading: {}\n", message::NAME, &args.input_file, e),
	};

	let mut f_str = String::new();
	match file.read_to_string(&mut f_str) {
		Ok(_) => (),
		Err(e) => return format!("{}: cannot open '{}' for reading: {}\n", message::NAME, &args.input_file, e),
	}

	let res: (Vec<pattern::Pattern>, String) = pattern::from_args(&args);
	if res.1 != "" {
		return res.1;
	}

	let patterns = res.0;

	for el in patterns {
		println!("{}", el);
	}

	return format!("{}", args);
}
