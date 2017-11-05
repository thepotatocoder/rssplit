extern crate lazy_static;
extern crate unindent;

pub static VERSION_NUM: &'static str = "0.1.0";
pub static NAME: &'static str = "rssplit";
pub static AUTHOR: &'static str = "thepotatocoder";

lazy_static! {
	pub static ref USAGE: String = {
		format!(
"Usage: {} [OPTION]... FILE PATTERN...
Output pieces of FILE separated by PATTERN(s) to files 'xx00', 'xx01', ...,
and output byte counts of each piece to standard output.

Read standard input if FILE is -

Mandatory arguments to long options are mandatory for short options too.
-b, --suffix-format=FORMAT  use sprintf FORMAT instead of %02d
-f, --prefix=PREFIX        use PREFIX instead of 'xx'
-k, --keep-files           do not remove output files on errors
	--suppress-matched     suppress the lines matching PATTERN
-n, --digits=DIGITS        use specified number of digits instead of 2
-s, --quiet, --silent      do not print counts of output file sizes
-z, --elide-empty-files    remove empty output files
	--help     display this help and exit
	--version  output version information and exit

Each PATTERN may be:
INTEGER            copy up to but not including specified line number
/REGEXP/[OFFSET]   copy up to but not including a matching line
%REGEXP%[OFFSET]   skip to, but not including a matching line
{{INTEGER}}          repeat the previous pattern specified number of times
{{*}}                repeat the previous pattern as many times as possible

A line OFFSET is a required '+' or '-' followed by a positive integer.
",
		NAME)
	};

	pub static ref VERSION: String = {
		format!(
"{} (GNU coreutils rewrite) {}
Copyright (C) 2017 {}
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Rewritten in Rust by {}
Originally written in C by Stuart Kemp and David MacKenzie.
",
		NAME, AUTHOR, VERSION_NUM, AUTHOR)
	};

	pub static ref TRYHELP: String = {
		format!(
"Try '{} --help' for more information.
",
		NAME)
	};
}	
