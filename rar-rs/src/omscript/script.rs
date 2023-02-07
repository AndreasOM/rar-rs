use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::{self, IResult};
use oml_game::system::System;

#[derive(Debug, Default, PartialEq)]
pub enum Literal {
	I128(i128),
	STRING(String),
	#[default]
	None,
}

#[derive(Debug, Default)]
pub enum OpCode {
	BlockStart,
	BlockEnd,
	Fn(u16),       // offset to name literal
	Call(u16, u8), // offset to target literal, number of parameters
	Literal(u16),
	#[default]
	End,
}

#[derive(Debug, Default)]
pub struct Script {
	code:     Vec<OpCode>,
	labels:   HashMap<String, usize>,
	literals: Vec<Literal>,
}

impl Script {
	pub fn from_asset(system: &mut System, name: &str) -> anyhow::Result<Script> {
		let dfs = system.default_filesystem_mut();
		// try yaml
		let name_omrs = format!("{}.omscript.rs", &name);
		if dfs.exists(&name_omrs) {
			let mut f = dfs.open(&name_omrs);
			let omrs = f.read_as_string();
			Self::load_from_str(&omrs)
		} else {
			// :TODO: create fallback?
			anyhow::bail!("Couldn't find script {}", name);
		}
	}

	pub fn find_label(&self, label: &str) -> Option<usize> {
		self.labels.get(label).copied()
	}

	pub fn get_op_code(&self, pc: usize) -> Option<&OpCode> {
		if pc >= self.code.len() {
			None
		} else {
			Some(&self.code[pc])
		}
	}

	pub fn get_literal(&self, i: usize) -> Option<&Literal> {
		if i >= self.literals.len() {
			None
		} else {
			Some(&self.literals[i])
		}
	}

	pub fn get_literal_str(&self, i: usize) -> Option<&str> {
		if let Some(l) = self.get_literal(i) {
			if let Literal::STRING(name) = l {
				return Some(&name);
			}
		}
		None
	}

	fn push_op_code(&mut self, op_code: OpCode) -> usize {
		self.code.push(op_code);
		self.code.len() - 1
	}

	fn add_label(&mut self, name: &str, pc: usize) -> anyhow::Result<()> {
		if self.labels.contains_key(name) {
			anyhow::bail!("Duplicated label {}", name)
		} else {
			self.labels.insert(name.to_string(), pc);
			Ok(())
		}
	}

	fn add_literal(&mut self, literal: Literal) -> usize {
		if let Some(pos) = self.literals.iter().position(|l| l == &literal) {
			pos
		} else {
			self.literals.push(literal);
			self.literals.len() - 1
		}
	}

	fn load_from_str(src: &str) -> anyhow::Result<Script> {
		tracing::debug!("Script source:\n{}", src);
		let mut s = Script::default();
		let items = item_parse_script(&src);
		tracing::debug!("items {:#?}", items);

		// convert to Opcodes, etc
		match items {
			Err(e) => {
				anyhow::bail!("Error while parsing script {:?}", e)
			},
			Ok(items) => {
				let (_rest, items) = items;
				//tracing::debug!("{:?}", items);
				for item in items {
					tracing::debug!("{:?}", item);
					match item {
						Item::Comment(_c) => {}, // skip
						Item::Fn { identifier, block } => {
							if let Item::Identifier(i) = *identifier {
								let l = s.add_literal(Literal::STRING(i.to_string()));
								let pc = s.push_op_code(OpCode::Fn(l as u16));
								//let Single::S(b) = a;
								s.add_label(i, pc)?;
							} else {
								unreachable!();
							}
							if let Item::Block(statements) = *block {
								s.push_op_code(OpCode::BlockStart);
								if let Item::Statements(statements) = *statements {
									for statement in statements {
										match statement {
											Item::Comment(_c) => {}, // skip
											Item::None => {},        // skip
											Item::Call {
												identifier,
												parameters,
											} => {
												if let Item::Identifier(i) = *identifier {
													let l = s.add_literal(Literal::STRING(
														i.to_string(),
													));
													let parameter_count = parameters.len();
													if parameter_count > 1 {
														unimplemented!();
													}
													s.push_op_code(OpCode::Call(
														l as u16,
														parameter_count as u8,
													));
													for parameter in parameters {
														match parameter {
															Item::Number(n) => {
																let l =
																	s.add_literal(Literal::I128(n));
																s.push_op_code(OpCode::Literal(
																	l as u16,
																));
															},
															Item::String(v) => {
																let l = s.add_literal(
																	Literal::STRING(v.to_string()),
																);
																s.push_op_code(OpCode::Literal(
																	l as u16,
																));
															},
															_ => unreachable!(),
														}
													}
												} else {
													unreachable!();
												}
											},
											_ => unreachable!(),
										}
									}
								} else {
									unreachable!();
								}

								s.push_op_code(OpCode::BlockEnd);
							} else {
								unreachable!();
							}
						},
						Item::None => {}, // skip
						i => {
							tracing::warn!("Unhandled {:?} on global scope", i);
						},
					}
				}
				Ok(s)
			},
		}

		//		Ok(s)
	}
}

#[derive(Debug, Default, Clone)]
enum Item<'a> {
	Comment(&'a str),
	Number(i128),
	String(&'a str),
	Identifier(&'a str),
	Call {
		identifier: Box<Item<'a>>,
		parameters: Vec<Item<'a>>,
	},
	Statements(Vec<Item<'a>>),
	Block(Box<Item<'a>>),
	Fn {
		identifier: Box<Item<'a>>,
		block:      Box<Item<'a>>,
	},
	#[default]
	None,
}

//use nom::{bytes::complete::tag, IResult};

fn parse_number(i: &str) -> IResult<&str, &str> {
	recognize(i128)(i)
}

fn item_parse_number(i: &str) -> IResult<&str, Item> {
	map(i128, |s: i128| Item::Number(s))(i)
}

fn parse_string(i: &str) -> IResult<&str, &str> {
	recognize(delimited(tag(r#"""#), is_not(r#"""#), tag(r#"""#)))(i)
}

fn item_parse_string(i: &str) -> IResult<&str, Item> {
	map(
		delimited(tag(r#"""#), is_not(r#"""#), tag(r#"""#)),
		|s: &str| Item::String(s),
	)(i)
}

fn parse_identifier(i: &str) -> IResult<&str, &str> {
	recognize(many1(alt((alpha1, tag("_")))))(i)
}

fn item_parse_identifier(i: &str) -> IResult<&str, Item> {
	map(
		recognize(many1(alt((alpha1, tag("_"))))), // Note: This needs some work to allow trailing numbers, and more
		|s: &str| Item::Identifier(s),
	)(i)
}

fn parse_literal(s: &str) -> IResult<&str, &str> {
	recognize(alt((parse_number, parse_string)))(s)
}

fn item_parse_literal(s: &str) -> IResult<&str, Item> {
	alt((item_parse_number, item_parse_string))(s)
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

fn item_parse_call(s: &str) -> IResult<&str, Item> {
	// :TODO: handle parameter list
	map(
		tuple((
			item_parse_identifier,
			multispace0,
			tag("("),
			multispace0,
			many0(item_parse_literal), // :TODO: make parameter list
			//item_parse_literal,	// :TODO: make parameter list
			multispace0,
			tag(")"),
			multispace0,
			tag(";"), // Note: The ';' should probably be part of the statements parser
		)),
		|v| Item::Call {
			identifier: Box::new(v.0),
			parameters: v.4,
		}, // :TODO: maybe better to extract the &str here?
	)(s)
}

fn parse_statements(s: &str) -> IResult<&str, &str> {
	recognize(many0(alt((multispace1, parse_comment, parse_call))))(s)
}

fn item_multispace1(i: &str) -> IResult<&str, Item> {
	map(multispace1, |_s| Item::None)(i)
}

fn item_parse_statements(s: &str) -> IResult<&str, Item> {
	map(
		many0(alt((item_multispace1, item_parse_comment, item_parse_call))),
		|v| Item::Statements(v),
	)(s)
}

fn parse_block(s: &str) -> IResult<&str, &str> {
	recognize(delimited(tag("{"), parse_statements, tag("}")))(s)
}

fn item_parse_block(s: &str) -> IResult<&str, Item> {
	map(delimited(tag("{"), item_parse_statements, tag("}")), |s| {
		Item::Block(Box::new(s))
	})(s)
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

fn item_parse_fn(s: &str) -> IResult<&str, Item> {
	map(
		tuple((
			tag("fn"),
			multispace1,
			item_parse_identifier,
			multispace0,
			tag("()"),
			multispace0,
			item_parse_block,
		)),
		|s| Item::Fn {
			identifier: Box::new(s.2),
			block:      Box::new(s.6),
		},
	)(s)
}

fn parse_comment(s: &str) -> IResult<&str, &str> {
	delimited(tag("//"), not_line_ending, line_ending)(s)
}

fn item_parse_comment(s: &str) -> IResult<&str, Item> {
	map(
		delimited(tag("//"), not_line_ending, line_ending),
		|s: &str| Item::Comment(s),
	)(s)
}

fn parse_something(s: &str) -> IResult<&str, &str> {
	alt((multispace1, parse_comment, parse_fn))(s)
}

fn item_parse_something(s: &str) -> IResult<&str, Item> {
	alt((item_multispace1, item_parse_comment, item_parse_fn))(s)
}

fn parse_script(s: &str) -> IResult<&str, Vec<&str>> {
	all_consuming(many0(parse_something))(s)
}

fn item_parse_script(s: &str) -> IResult<&str, Vec<Item>> {
	all_consuming(many0(item_parse_something))(s)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn can_parse_comment() -> anyhow::Result<()> {
		let r = parse_comment("// comment\n");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = parse_comment("// comment   \n");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = item_parse_comment("// comment   \n");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_number() -> anyhow::Result<()> {
		let r = item_parse_number("1234");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}
	#[test]
	fn can_parse_string() -> anyhow::Result<()> {
		let r = item_parse_string(r#""test abc 123 !?:""#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_literal() -> anyhow::Result<()> {
		let r = item_parse_literal("1234");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = item_parse_literal(r#""test string literal // not a comment""#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_identifier() -> anyhow::Result<()> {
		//		let r = item_parse_identifier("test_abc_123");	// :TODO:
		let r = item_parse_identifier("test_abc");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_fn() -> anyhow::Result<()> {
		let r = item_parse_fn("fn test() {}");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_block() -> anyhow::Result<()> {
		let r = item_parse_block("{ test(); }");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		//		let r = parse_block("{\n\\ test\ntest(); \n}");
		let r = item_parse_block("{\n\ntest(); \n}");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_call() -> anyhow::Result<()> {
		let r = item_parse_call("test();");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = item_parse_call("test(10);");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = item_parse_call(r#"test ( "test" ) ;"#);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	#[test]
	fn can_parse_statements() -> anyhow::Result<()> {
		let r = item_parse_statements("test();");
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		let r = item_parse_statements(
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

		let r = item_parse_script(s);
		eprintln!("{:?}", r);
		assert!(r.is_ok());
		Ok(())
	}

	use super::Script;
	#[test]
	fn can_load_from_str() -> anyhow::Result<()> {
		let s = r#"

// comment
fn run() {	// main function
	wait_frames(10);
	take_screenshot("test");
	app_quit();
}
"#;
		eprintln!("{:?}", s);

		let s = Script::load_from_str(s);
		eprintln!("{:?}", s);
		assert!(s.is_ok());
		Ok(())
	}
}
