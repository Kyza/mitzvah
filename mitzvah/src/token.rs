use crate::parser::ParseError;
use crate::pm::{token_stream::IntoIter, TokenStream};
use std::iter::Peekable;

pub type TokenIterator = Peekable<IntoIter>;

pub trait Token {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized;

	fn peek(
		stream: &mut TokenIterator,
		advance: usize,
	) -> Result<Self, ParseError>
	where
		Self: Sized,
	{
		let mut forked_stream = stream.clone();
		#[cfg(feature = "iter_advance_by")]
		let _ = forked_stream.advance_by(advance);
		#[cfg(not(feature = "iter_advance_by"))]
		for _ in 0..advance {
			forked_stream.next();
		}
		Self::parse(&mut forked_stream)
	}

	fn reconstruct(&self) -> TokenStream {
		TokenStream::default()
	}
}
