use crate::parser::ParseError;
use crate::pm::{LexError, Literal, TokenTree};
use crate::token::{Token, TokenIterator};
use std::any::type_name;
use std::str::FromStr;

#[cfg(feature = "proc-macro2")]
use syn::Lit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
	Byte,
	Char,
	Integer,
	Float,
	Str,
	StrRaw(u8),
	ByteStr,
	ByteStrRaw(u8),
	CStr,
	CStrRaw(u8),
	Bool,
	Err,
}

fn get_literal_raw_depth(literal: &str) -> u8 {
	let mut depth = 0;
	for byte in literal.bytes() {
		match byte {
			b'#' => {
				depth += 1;
			}
			b'"' => {
				break;
			}
			_ => (),
		}
	}
	depth
}

pub trait MitzvahLiteralExt {
	fn kind(&self) -> LiteralKind;
	fn suffix(&self) -> Option<Box<str>>;
}

#[cfg(feature = "proc-macro2")]
impl MitzvahLiteralExt for Literal {
	fn kind(&self) -> LiteralKind {
		match Lit::new(self.clone()) {
			Lit::Str(lit_str) => {
				let value = lit_str.token().to_string();
				if value.starts_with('r') {
					LiteralKind::StrRaw(get_literal_raw_depth(&value))
				} else {
					LiteralKind::Str
				}
			}
			Lit::ByteStr(lit_byte_str) => {
				let value = lit_byte_str.token().to_string();
				if value.starts_with("br") {
					LiteralKind::ByteStrRaw(get_literal_raw_depth(&value))
				} else {
					LiteralKind::ByteStr
				}
			}
			Lit::Byte(_) => LiteralKind::Byte,
			Lit::Char(_) => LiteralKind::Char,
			Lit::Int(_) => LiteralKind::Integer,
			Lit::Float(_) => LiteralKind::Float,
			Lit::Bool(_) => LiteralKind::Bool,
			_ => LiteralKind::Err,
		}
	}

	fn suffix(&self) -> Option<Box<str>> {
		let lit = Lit::new(self.clone());
		let suffix = lit.suffix();
		if suffix.len() == 0 {
			None
		} else {
			Some(Box::from(suffix))
		}
	}
}

#[cfg(not(feature = "proc-macro2"))]
impl MitzvahLiteralExt for Literal {
	fn kind(&self) -> LiteralKind {
		let string = format!("{:?}", self);

		println!("{}", string);

		let pattern = "kind: ";
		let start = match string.find(pattern).ok_or(LiteralKind::Err) {
			Ok(start) => start + pattern.len(),
			Err(_) => {
				return LiteralKind::Err;
			}
		};
		let end = match string[start..].find(|c| c == ',' || c == '(') {
			Some(end) => start + end,
			None => {
				return LiteralKind::Err;
			}
		};

		match &string[start..end] {
			"Char" => LiteralKind::Char,
			"Integer" => LiteralKind::Integer,
			"Float" => LiteralKind::Float,
			"StrRaw" => LiteralKind::StrRaw(get_literal_raw_depth(&string)),
			"Str" => LiteralKind::Str,
			"ByteStrRaw" => {
				LiteralKind::ByteStrRaw(get_literal_raw_depth(&string))
			}
			"ByteStr" => LiteralKind::ByteStr,
			"Byte" => LiteralKind::Byte,
			"CStrRaw" => LiteralKind::CStrRaw(get_literal_raw_depth(&string)),
			"CStr" => LiteralKind::CStr,
			_ => LiteralKind::Err,
		}
	}

	fn suffix(&self) -> Option<Box<str>> {
		let string = format!("{:?}", self);

		let pattern = "suffix: ";
		let mut start = match string.find(pattern).ok_or(LiteralKind::Err) {
			Ok(start) => start + pattern.len(),
			Err(_) => {
				return None;
			}
		};
		if string[start..].starts_with("None") {
			return None;
		}
		start += 6; // Length of `Some("`
		let end = match string[start..].find('"') {
			Some(end) => start + end,
			None => {
				return None;
			}
		};

		Some(Box::from(&string[start..end]))
	}
}

impl Token for Literal {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let result = match parser.peek() {
			Some(TokenTree::Literal(value)) => Ok(value.clone()),
			Some(value) => Err(ParseError::UnexpectedToken {
				span: value.span(),
				expected_token: type_name::<Self>(),
			}),
			None => Err(ParseError::EndOfStream),
		};
		if result.is_ok() {
			parser.next();
		}
		result
	}
}

#[test]
fn literal_kinds() -> Result<(), LexError> {
	assert_eq!(Literal::from_str("b'a'")?.kind(), LiteralKind::Byte);
	assert_eq!(Literal::from_str("'a'")?.kind(), LiteralKind::Char);
	assert_eq!(Literal::from_str("1")?.kind(), LiteralKind::Integer);
	assert_eq!(Literal::from_str("1.0")?.kind(), LiteralKind::Float);
	assert_eq!(Literal::from_str(r#"":)""#)?.kind(), LiteralKind::Str);
	assert_eq!(
		Literal::from_str(r##"r#":)"#"##)?.kind(),
		LiteralKind::StrRaw(1)
	);
	assert_eq!(Literal::from_str(r#"b":)""#)?.kind(), LiteralKind::ByteStr);
	assert_eq!(
		Literal::from_str(r##"br#":)"#"##)?.kind(),
		LiteralKind::ByteStrRaw(1)
	);

	// assert_eq!(Literal::from_str("true")?.kind(), LiteralKind::Bool);
	// assert_eq!(Literal::from_str("false")?.kind(), LiteralKind::Bool);

	Ok(())
}

#[test]
fn literal_suffixes() -> Result<(), LexError> {
	assert_eq!(Literal::from_str("1")?.suffix(), None);
	assert_eq!(
		Literal::from_str("1usize")?.suffix().unwrap(),
		"usize".into()
	);

	Ok(())
}

#[test]
fn raw_literal_depth() {
	assert_eq!(get_literal_raw_depth(r##"r#""#"##), 1);
}
