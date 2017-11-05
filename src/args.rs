
extern crate clap;

use std::fmt;
use std::str::FromStr;
use std::string::String;

use message;

pub struct Args {
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
		"\tInput: '{}'\n\tFile Pattern: {:?}\n\tSuffix Format: {}\n\tPrefix: {}\n\tKeep files: {}\n\t\
		Suppress matched: {}\n\tDigits: {}\n\tQuiet: {}\n\tElide empty files: {}\n",
			self.input_file, self.file_pattern, self.suffix_format, self.prefix, self.keep_files,
			self.suppress_matched, self.digits, self.quiet, self.elide_empty_files)
	}
}

impl Args {
	//TODO: Handle errors better
	pub fn from_matches(matches: &clap::ArgMatches) -> (Args, String) {
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
			None => return (args, format!("{}: missing input file", message::NAME)),
		};

		//TODO: Error Handling?
    	let res = match matches.values_of("FILE_PATTERN") {
			Some(a) => a,
			None => return (args, format!("{}: missing file pattern", message::NAME)),
		};

		args.file_pattern = res.map(String::from).collect();

		args.suffix_format = match matches.value_of("SUFFIX_FORMAT") {
	        Some(a) => a.to_string(),
	        None => {
				if matches.occurrences_of("SUFFIX_FORMAT") == 1 {
					return (args, format!("{}: error with suffix format", message::NAME));
				} else {
					String::from("%02d")
				}
			}   
		};
	
		args.prefix = match matches.value_of("PREFIX") {
	        Some(a) => a.to_string(),
			None => {
				if matches.occurrences_of("PREFIX") == 1 {
					return (args, format!("{}: error with prefix", message::NAME));
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
	            Err(_) => return (args, format!("{}: invalid digits", message::NAME)),
	        };
	
		if matches.occurrences_of("QUIET") != 0 {
			args.quiet = true;
		}

		if matches.occurrences_of("ELIDE_EMPTY_FILES") != 0 {
			args.elide_empty_files = true;
		}

	    (args, format!(""))
	}
}
