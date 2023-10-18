use crate::parser::ParseError;
use crate::pm::{Ident, TokenTree};
use crate::token::{Token, TokenIterator};
use std::any::type_name;

pub trait MitzvahIdentExt {
	fn ident(&self) -> String;
}
impl MitzvahIdentExt for Ident {
	fn ident(&self) -> String {
		self.to_string()
	}
}

impl Token for Ident {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let result = match parser.peek() {
			Some(TokenTree::Ident(value)) => Ok(value.clone()),
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
