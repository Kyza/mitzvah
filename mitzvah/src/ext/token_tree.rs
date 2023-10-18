use crate::parser::ParseError;
use crate::pm::TokenTree;
use crate::token::{Token, TokenIterator};

pub trait MitzvahTokenTreeExt {}
impl MitzvahTokenTreeExt for TokenTree {}

impl Token for TokenTree {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let result = match parser.peek() {
			Some(value) => Ok(value.clone()),
			None => Err(ParseError::EndOfStream),
		};
		if result.is_ok() {
			parser.next();
		}
		result
	}
}
