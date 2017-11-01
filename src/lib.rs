
extern crate clap;
extern crate grep;

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::string::String;

pub static USAGE: &'static str = include_str!("str/usage.txt");
pub static VERSION: &'static str = include_str!("str/version.txt");
pub static TRYHELP: &'static str = include_str!("str/tryhelp.txt");
pub static ERROR: &'static str = "csplit: {}\nTry 'csplit --help' for more information.\n";

struct Args {
    pub input_file: String,
    pub file_pattern: Vec<String>,
    pub suffix_format: String,
    pub prefix: String,
    pub keep_files: bool,
    pub suppress_matched: bool,
    pub digits: i32,
    pub quiet: bool,
    pub elide_empty_files: bool,
    //help: bool,
    //version: bool,
}

impl fmt::Display for Args {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		"\tInput: '{:?}'\n\tFile Pattern: {:?}\n\tSuffix Format: {}\n\tPrefix: {}\n\tKeep files: {}\n\t\
		Suppress matched: {}\n\tDigits: {}\n\tQuiet: {}\n\tElide empty files: {}\n",
			self.input_file, self.file_pattern, self.suffix_format, self.prefix, self.keep_files,
			self.suppress_matched, self.digits, self.quiet, self.elide_empty_files)
	}
}

impl Args {
	//TODO: Handle errors better
	pub fn from_matches(matches: &clap::ArgMatches) -> Args {
		let mut args = Args {
 	       input_file: String::default(),
 	       file_pattern: Vec::new(),
 	       suffix_format: String::from("%02d"),
 	       prefix: String::from("xx"),
 	       keep_files: false,
 	       suppress_matched: false,
 	       digits: 2,
 	       quiet: false,
 	       elide_empty_files: false,
 	   };

    	args.input_file = match matches.value_of("INPUT") {
			Some(a) => String::from(a),
			None => String::from("this will never happen right?"),
		};

		//TODO: Error Handling?
    	args.file_pattern = matches.values_of("FILE_PATTERN").unwrap().map(String::from).collect();

		args.suffix_format = match matches.value_of("SUFFIX_FORMAT") {
	        Some(a) => a.to_string(),
	        None => {
				if matches.occurrences_of("SUFFIX_FORMAT") == 1 {
					String::from("this will never happen right?")
				} else {
					String::from("%02d")
				}
			}   
		};
	
		args.prefix = match matches.value_of("PREFIX") {
	        Some(a) => a.to_string(),
			None => {
				if matches.occurrences_of("PREFIX") == 1 {
					String::from("this will never happen right?")
				} else {
					String::from("xx")
				}
			}

	    };

		if matches.occurrences_of("KEEP_FILES") != 0 {
			args.keep_files = true;
		}
	
		if matches.occurrences_of("SUPPRESS_MATCHED") != 0 {
			args.suppress_matched = true;
		}

	    args.digits = match FromStr::from_str(
	        match matches.value_of("DIGITS"){
	            Some(s) => s,
	            None => "2",
	        }) {
	            Ok(a) => a,
	            Err(_) => panic!("Invalid number"),
	        };
	
		if matches.occurrences_of("QUIET") != 0 {
			args.quiet = true;
		}

		if matches.occurrences_of("ELIDE_EMPTY_FILES") != 0 {
			args.elide_empty_files = true;
		}

	    args
	}
}

pub fn uumain (matches: clap::ArgMatches) -> String {
	//Is there a way to check which came first in clap?
	//Csplit adresses each of these flags by order of arrival
	let args = Args::from_matches(&matches);

	if matches.occurrences_of("VERSION") != 0 {
		return VERSION.to_string();
	}
	if matches.occurrences_of("HELP") != 0 {
		return USAGE.to_string();
	}

	if matches.occurrences_of("INPUT") == 0 {
		return format!("csplit: missing operand\n{}", TRYHELP.to_string());
	}

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

	//Pattern matching
	//		INTEGER				copy up to but not including specified line number
	//		/REGEXP/[OFFSET]	copy up to but not including a matching line
	//		%REGEXP%[OFFSET]	skip to, but not including a matching line
	//		{INTEGER}			repeat the previous pattern specified number of times
	//		{*}					repeat the previous pattern as many times as possible
	//A line OFFSET is a required '+' or '-' followed by a positive integer.

	for patt in &args.file_pattern {
		if patt.chars().all(|c| c.is_numeric()) {
			println!("Num\tPattern: {}", patt);
		} else if patt.starts_with("/") {
			println!(" /\tPattern: {}", patt);
		} else if patt.starts_with("%") {
			println!(" %\tPattern: {}", patt);
		} else if patt.starts_with("{") {
			//Repeat pattern
			println!("Rep\tPattern: {}", patt);
		}
		else {
			//Invalid format
			println!("Bad\tPattern: {}", patt);
		}
	}
	
	//let builder = grep::GrepBuilder::new("");

	return format!("{}", args);
}
