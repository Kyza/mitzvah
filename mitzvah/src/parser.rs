use crate::pm::Span;
use crate::token::{Token, TokenIterator};

#[derive(Debug)]
pub enum ParseError {
	UnexpectedToken {
		span: Span,
		expected_token: &'static str,
	},
	EndOfStream,
	Unknown,
}

pub trait MitzvahParser {
	fn parse<T>(&mut self) -> Result<T, ParseError>
	where
		T: Token;
	fn peek<T>(&mut self, advance: usize) -> Result<T, ParseError>
	where
		T: Token;

	fn has_next(&mut self) -> bool;
}
impl MitzvahParser for TokenIterator {
	fn parse<T>(&mut self) -> Result<T, ParseError>
	where
		T: Token,
	{
		T::parse(self)
	}

	fn peek<T>(&mut self, advance: usize) -> Result<T, ParseError>
	where
		T: Token,
	{
		T::peek(self, advance)
	}

	fn has_next(&mut self) -> bool {
		self.peek().is_some()
	}
}
