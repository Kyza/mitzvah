use crate::parser::ParseError;
use crate::pm::{Punct, TokenTree};
use crate::token::{Token, TokenIterator};
use std::any::type_name;

pub trait MitzvahPunctExt {
	fn ch(&self) -> char;
}
impl MitzvahPunctExt for Punct {
	fn ch(&self) -> char {
		self.as_char()
	}
}

impl Token for Punct {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let result = match parser.peek() {
			Some(TokenTree::Punct(value)) => Ok(value.clone()),
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
