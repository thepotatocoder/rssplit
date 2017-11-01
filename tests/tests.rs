
#[macro_use]
extern crate clap;
extern crate csplit;

#[cfg(test)]
mod test_args {
	use clap::App;
	use csplit;

	#[test]
	fn no_arguments() {
		let yaml = load_yaml!("../src/csplit.yml");
		let arg_vec = vec!["csplit"];
		let matches = App::from_yaml(yaml).get_matches_from(arg_vec);

		assert_eq!(csplit::uumain(matches), format!("csplit: missing operand\n{}", csplit::TRYHELP.to_string()));
	}

	#[test]
	fn only_input() {
		let yaml = load_yaml!("../src/csplit.yml");
		let arg_vec = vec!["csplit", "input"];
		let matches = App::from_yaml(yaml).get_matches_from(arg_vec);

		assert_eq!(csplit::uumain(matches), format!("csplit: missing operand after 'input'\n{}", csplit::TRYHELP.to_string()));
	}
}