
#[macro_use]
extern crate clap;
extern crate rssplit;

#[cfg(test)]
mod test_args {
	use clap::App;
    use rssplit;

	macro_rules! test {
		($name: ident, $vec: expr, $eq: expr) => {
			#[test]
			fn $name() {
				let yaml = load_yaml!("../src/bin/csplit.yml");
				let matches = App::from_yaml(yaml).get_matches_from($vec);

				assert_eq!(rssplit::uumain(matches), $eq);
			}
		};
	}
	
	test!(no_arguments, vec![""], format!("{}: missing operand\n{}", rssplit::message::NAME, rssplit::message::TRYHELP.to_string()));
	test!(only_input, vec!["", "input"], format!("{}: missing operand after 'input'\n{}", rssplit::message::NAME, rssplit::message::TRYHELP.to_string()));
}
