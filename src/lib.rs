
extern crate clap;
extern crate grep;

mod args;
mod pattern;

use std::fs::File;
use std::io::Read;
use std::string::String;

pub static USAGE: &'static str = include_str!("str/usage.txt");
pub static VERSION: &'static str = include_str!("str/version.txt");
pub static TRYHELP: &'static str = include_str!("str/tryhelp.txt");
pub static ERROR: &'static str = "csplit: {}\nTry 'csplit --help' for more information.\n";

pub fn uumain (matches: clap::ArgMatches) -> String {
	//Is there a way to check which came first in clap?
	//Csplit adresses each of these flags by order of arrival

	if matches.occurrences_of("VERSION") != 0 {
		return VERSION.to_string();
	} else if matches.occurrences_of("HELP") != 0 {
		return USAGE.to_string();
	} else if matches.occurrences_of("INPUT") == 0 {
		return format!("csplit: missing operand\n{}", TRYHELP.to_string());
	}
	
	let args = args::Args::from_matches(&matches);

	if (matches.occurrences_of("INPUT") == 1) && (matches.occurrences_of("FILE_PATTERN") == 0) {
		return format!("csplit: missing operand after '{}'\n{}", args.input_file, TRYHELP.to_string());
	}

	//Check if input file really exists
	let mut file = match File::open(&args.input_file) {
		Ok(f) => f,
		Err(e) => return format!("csplit: cannot open '{}' for reading: {}\n", &args.input_file, e),
	};

	let mut f_str = String::new();
	match file.read_to_string(&mut f_str) {
		Ok(_) => (),
		Err(e) => return format!("csplit: cannot open '{}' for reading: {}\n", &args.input_file, e),
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
