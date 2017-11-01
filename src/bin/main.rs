/* A rust re-write of csplit named rssplit */

#[macro_use]
extern crate clap;
extern crate csplit;

use clap::App;

fn main() {

	//Set program global settings?
	//Program name, locale

	//Parse command-line arguments
	let yaml = load_yaml!("csplit.yml");
	let matches = App::from_yaml(yaml).get_matches();

	//Make sure arguments are valid (e.g.: digits is a valid i32)
    print!("{}", csplit::uumain(matches));
	//Output file name related henanigans

	//<C> set_input_file()

	//<C> parse_patterns()

	//Signal related stuff??
}
