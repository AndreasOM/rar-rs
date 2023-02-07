use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{self, IResult};
use oml_game::system::System;

#[derive(Debug, Default)]
enum OpCode {
	BlockStart,
	BlockEnd,
	Fn(u16),   // offset to name literal
	Call(u16), // offset to target literal
	#[default]
	End,
}

#[derive(Debug, Default)]
pub struct Script {
	code:     Vec<OpCode>,
	labels:   HashMap<String, usize>,
	literals: Vec<String>,
}

impl Script {
	pub fn from_asset(system: &mut System, name: &str) -> anyhow::Result<Script> {
		let dfs = system.default_filesystem_mut();
		// try yaml
		let name_omrs = format!("{}.omscript.rs", &name);
		if dfs.exists(&name_omrs) {
			let mut f = dfs.open(&name_omrs);
			let omrs = f.read_as_string();
			tracing::debug!("Script source:\n{}", omrs);
			let s = Script::default();
			let ast = parse_script(&omrs);
			tracing::debug!("ast {:#?}", ast);
			Ok(s)
		} else {
			// :TODO: create fallback?
			anyhow::bail!("Couldn't find script {}", name);
		}
	}
}

//use nom::{bytes::complete::tag, IResult};

fn parse_number(i: &str) -> IResult<&str, &str> {
	recognize(i128)(i)
}

fn parse_string(i: &str) -> IResult<&str, &str> {
	recognize(delimited(tag(r#"""#), is_not(r#"""#), tag(r#"""#)))(i)
}

fn parse_identifier(i: &str) -> IResult<&str, &str> {
	recognize(many1(alt((alpha1, tag("_")))))(i)
}

fn parse_literal(s: &str) -> IResult<&str, &str> {
	recognize(alt((parse_number, parse_string)))(s)
}

fn parse_call(s: &str) -> IResult<&str, &str> {
	// :TODO: handle parameter list
	recognize(tuple((
		parse_identifier,
		multispace0,
		tag("("),
		multispace0,
		many0(parse_literal),
		multispace0,
		tag(")"),
		multispace0,
		tag(";"), // Note: The ';' should probably be part of the statements parser
	)))(s)
}

fn parse_statements(s: &str) -> IResult<&str, &str> {
	recognize(many0(alt((multispace1, parse_comment, parse_call))))(s)
}

fn parse_block(s: &str) -> IResult<&str, &str> {
	recognize(delimited(tag("{"), parse_statements, tag("}")))(s)
}
fn parse_fn(s: &str) -> IResult<&str, &str> {
	recognize(tuple((
		tag("fn"),
		multispace1,
		parse_identifier,
		multispace0,
		tag("()"),
		multispace0,
		parse_block,
	)))(s)
}

fn parse_comment(s: &str) -> IResult<&str, &str> {
	delimited(tag("//"), not_line_ending, line_ending)(s)
}

fn parse_something(s: &str) -> IResult<&str, &str> {
	alt((multispace1, parse_comment, parse_fn))(s)
}
fn parse_script(s: &str) -> IResult<&str, Vec<&str>> {
	all_consuming(many0(parse_something))(s)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn can_parse_comment() -> anyhow::Result<()> {
		let r = parse_script("// comment\n");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_script("\n\n// comment\n\n");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_number() -> anyhow::Result<()> {
		let r = parse_number("1234");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}
	#[test]
	fn can_parse_string() -> anyhow::Result<()> {
		let r = parse_string(r#""test abc 123 !?:""#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_literal() -> anyhow::Result<()> {
		let r = parse_literal("1234");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_literal(r#""test string literal // not a comment""#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_fn() -> anyhow::Result<()> {
		let r = parse_fn("fn test() {}");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_block() -> anyhow::Result<()> {
		let r = parse_block("{ test(); }");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		//		let r = parse_block("{\n\\ test\ntest(); \n}");
		let r = parse_block("{\n\ntest(); \n}");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_call() -> anyhow::Result<()> {
		let r = parse_call("test();");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_call("test(10);");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_call(r#"test ( "test" ) ;"#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_statements() -> anyhow::Result<()> {
		let r = parse_statements("test();");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_statements(
			r#"
test(10);test( "forty 7" );
test () ;
"#,
		);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_script() -> anyhow::Result<()> {
		let s = r#"

// comment
fn run() {	// main function
	wait_frames(10);
	take_screenshot("test");
	app_quit();
}
"#;
		eprintln!("{:?}", s);

		let r = parse_script(s);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}
}
