#[cfg(not(feature = "proc-macro2"))]
extern crate proc_macro;

use crate::parser::ParseError;
use crate::token::TokenIterator;
use pm::TokenStream;

pub trait Macro {
	/// This function should take the source token stream as input and output an instance of Self with the processed data attached.
	fn parse(token_stream: TokenIterator) -> Result<Self, ParseError>
	where
		Self: Sized;

	/// This function should construct a new token stream from the data created by [Macro::parse].
	fn transform(data: Self) -> TokenStream;
}

pub mod ext;
pub mod parser;
pub mod token;

/// These are re-exports of the proc macro implementation Mitzvah is using at compile-time.
///
/// Use `mitzvah::pm::Type` to access types instead of `proc_macro::Type` to ensure your
/// tests work properly.
///
/// ## Features
///
/// `not(feature = "proc-macro2")` will use `proc_macro`.
///
/// `feature = "proc-macro2"` will use `proc_macro2`.
///
/// Since `proc_macro` isn't available outside of `proc-macro = true` crates,
/// you can use the following in your `Config.toml` to automatically enable
/// `feature = "proc-macro2"` for tests, but not for your macros in the real
/// world.
/// ```toml
/// [dependencies]
/// mitzvah = ".."
///
/// [dev-dependencies]
/// mitzvah = { version = "..", features = ["proc-macro2"] }
/// ```
pub mod pm {
	#[cfg(not(feature = "proc-macro2"))]
	#[doc(inline)]
	pub use proc_macro::*;
	#[cfg(feature = "proc-macro2")]
	#[doc(inline)]
	pub use proc_macro2::*;
}
