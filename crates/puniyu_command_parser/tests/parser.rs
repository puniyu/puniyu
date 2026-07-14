use puniyu_command_parser::{CommandParser, Error};
use puniyu_command_types::Arg;

fn default_parser() -> CommandParser {
	CommandParser::new()
}

fn bot_parser() -> CommandParser {
	CommandParser::builder().alias("@bot").alias("/bot").prefix("!").build()
}

fn hello_positional_args() -> Vec<Arg<'static>> {
	vec![Arg::string("name").positional().required()]
}

fn hello_named_args() -> Vec<Arg<'static>> {
	vec![Arg::string("name").named().required()]
}

fn calc_args() -> Vec<Arg<'static>> {
	vec![
		Arg::int("a").positional().required(),
		Arg::int("b").positional().required(),
		Arg::bool("verbose").named().short('v').long("verbose"),
		Arg::string("mode").named().short('m').long("mode"),
	]
}

#[test]
fn strips_bot_alias() {
	let result = bot_parser().parse("@bot hello Alice").unwrap();
	assert_eq!(result.command(), "hello");
	assert_eq!(result.tokens(), &["Alice"]);
}

#[test]
fn strips_prefix() {
	let parser = CommandParser::builder().prefix("!").build();
	let result = parser.parse("!hello Alice").unwrap();
	assert_eq!(result.command(), "hello");
}

#[test]
fn strips_alias_and_prefix() {
	let result = bot_parser().parse("@bot !hello Alice").unwrap();
	assert_eq!(result.command(), "hello");
	assert_eq!(result.tokens(), &["Alice"]);
}

#[test]
fn keeps_original_when_no_match() {
	let result = bot_parser().parse("hello Alice").unwrap();
	assert_eq!(result.command(), "hello");
}

#[test]
fn parses_positional_args() {
	let result = default_parser().parse("hello Alice").unwrap();
	let args = result.parse_args(&hello_positional_args()).unwrap();
	assert_eq!(args["name"][0].as_str(), Some("Alice"));
}

#[test]
fn parses_positional_ints() {
	let result = default_parser().parse("calc 10 20").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["a"][0].as_int(), Some(10));
	assert_eq!(args["b"][0].as_int(), Some(20));
}

#[test]
fn parses_long_named_arg() {
	let result = default_parser().parse("hello --name Alice").unwrap();
	let args = result.parse_args(&hello_named_args()).unwrap();
	assert_eq!(args["name"][0].as_str(), Some("Alice"));
}

// ── 命名参数 short ──────────────────────────────────────

#[test]
fn parses_short_named_arg() {
	let result = default_parser().parse("calc 1 2 -m fast").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["mode"][0].as_str(), Some("fast"));
}

#[test]
fn parses_bool_flag_without_value() {
	let result = default_parser().parse("calc 1 2 --verbose").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["verbose"][0].as_bool(), Some(true));
}

#[test]
fn parses_bool_short_flag() {
	let result = default_parser().parse("calc 1 2 -v").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["verbose"][0].as_bool(), Some(true));
}

#[test]
fn parses_bool_flag_with_value() {
	let result = default_parser().parse("calc 1 2 --verbose false").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["verbose"][0].as_bool(), Some(false));
}

#[test]
fn parses_mixed_positional_and_named() {
	let result = default_parser().parse("calc 5 3 -v -m fast").unwrap();
	let args = result.parse_args(&calc_args()).unwrap();
	assert_eq!(args["a"][0].as_int(), Some(5));
	assert_eq!(args["b"][0].as_int(), Some(3));
	assert_eq!(args["verbose"][0].as_bool(), Some(true));
	assert_eq!(args["mode"][0].as_str(), Some("fast"));
}

#[test]
fn handles_quoted_values() {
	let result = default_parser().parse(r#"hello --name "Alice in Wonderland""#).unwrap();
	let args = result.parse_args(&hello_named_args()).unwrap();
	assert_eq!(args["name"][0].as_str(), Some("Alice in Wonderland"));
}

#[test]
fn collects_duplicate_positional_as_vec() {
	let args = vec![Arg::string("item").positional()];
	let result = default_parser().parse("cmd a b c").unwrap();
	let args = result.parse_args(&args).unwrap();
	let items = args["item"].iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>();
	assert_eq!(items, vec!["a", "b", "c"]);
}

#[test]
fn errors_on_empty_input() {
	let result = default_parser().parse("");
	assert!(matches!(result, Err(Error::EmptyInput)));
}

#[test]
fn errors_on_missing_required() {
	let result = default_parser().parse("hello").unwrap().parse_args(&hello_named_args());
	assert!(matches!(result, Err(Error::MissingRequired { .. })));
}

#[test]
fn errors_on_invalid_int() {
	let result = default_parser().parse("calc abc 2").unwrap().parse_args(&calc_args());
	assert!(matches!(result, Err(Error::InvalidValue { .. })));
}

#[test]
fn errors_on_unknown_named_arg() {
	let result = default_parser()
		.parse("hello --name Alice --unknown val")
		.unwrap()
		.parse_args(&hello_named_args());
	assert!(matches!(result, Err(Error::UnknownArgument { .. })));
}

#[test]
fn errors_on_too_many_positional() {
	let result = default_parser().parse("hello a b").unwrap().parse_args(&hello_named_args());
	assert!(matches!(result, Err(Error::TooManyValues { .. })));
}
