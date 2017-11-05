
extern crate grep;

use args;

use std::fmt;
use std::str::FromStr;

//Pattern matching
//		INTEGER				copy up to but not including specified line number
//		/REGEXP/[OFFSET]	copy up to but not including a matching line
//		%REGEXP%[OFFSET]	skip to, but not including a matching line
//		{INTEGER}			repeat the previous pattern specified number of times
//		{*}					repeat the previous pattern as many times as possible
//A line OFFSET is a required '+' or '-' followed by a positive integer.

pub struct Pattern {
	pub id: usize,
	pub repeats_left: i32,
	pub regex: grep::Grep,
	pub repeater: bool,
}

impl fmt::Display for Pattern {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		"Pattern:\n\tID: {}\n\tRepeats left: {}\n\tRegex: {:?}\n\tRepeater: {}\n",
		self.id, self.repeats_left, self.regex, self.repeater)
	}
}

pub fn from_args(args: &args::Args) -> (Vec<Pattern>, String) {
	let mut patterns: Vec<Pattern> = Vec::new();
		
		for (i, patt) in args.file_pattern.iter().enumerate() {
		if patt.starts_with("{") {
			if (i == 0) || (patterns[i-1].repeater) {
				return (patterns, format!("csplit: '{}': invalid pattern\n", patt));
			}

			//How does this work?
			let chars_to_trim: &[char] = &['{', '}'];
			let s = patt.trim_matches(chars_to_trim);

			if s == "*" {
				patterns[i-1].repeats_left = -1;
			}

			if s.chars().all(|c| c.is_numeric()) {
				patterns[i-1].repeats_left = FromStr::from_str(s).unwrap();
			}

			let regex = patterns[i-1].regex.clone();

			patterns.push(Pattern {
				id: i,
				repeats_left: 0,
				//Regex value doesn't matter.
				//Maybe find a better way?
				regex,
				repeater: true
			});
		} else {
			let regex = match grep::GrepBuilder::new(patt).build() {
				Ok(r) => r,
				Err(_) => panic!("Could not compile regex"),
			};

			patterns.push(Pattern {
				id: i,
				repeats_left: 1,
				regex,
				repeater: false
			});
		}
	}

	(patterns, "".to_string())
}