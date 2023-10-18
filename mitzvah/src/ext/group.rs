use crate::parser::ParseError;
use crate::pm::{Group, TokenTree};
use crate::token::{Token, TokenIterator};
use std::any::type_name;

pub trait MitzvahGroupExt {}
impl MitzvahGroupExt for Group {}

impl Token for Group {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let result = match parser.peek() {
			Some(TokenTree::Group(value)) => Ok(value.clone()),
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
