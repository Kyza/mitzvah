use mitzvah::{
	parser::{MitzvahParser, ParseError},
	pm::{Literal, Punct, Spacing, Span, TokenStream},
	token::{Token, TokenIterator},
	Macro,
};
use std::any::type_name;

#[derive(Debug)]
struct MultiPunct {
	pub puncts: Vec<Punct>,
}

impl Token for MultiPunct {
	fn parse(parser: &mut TokenIterator) -> Result<Self, ParseError> {
		let mut punctuations = vec![];
		while parser.has_next() {
			let punct = parser.parse::<Punct>()?;
			let alone = punct.spacing() == Spacing::Alone;
			punctuations.push(punct);
			if alone {
				break;
			}
		}
		if punctuations.is_empty() {
			Err(ParseError::UnexpectedToken {
				span: Span::call_site(),
				expected_token: type_name::<Self>(),
			})
		} else {
			Ok(MultiPunct {
				puncts: punctuations,
			})
		}
	}
}

struct CoolMacro {
	data: TokenStream,
}

impl Macro for CoolMacro {
	fn parse(mut tokens: TokenIterator) -> Result<CoolMacro, ParseError> {
		let lit = tokens.parse::<Literal>()?;

		println!("{:?}", lit);
		println!("{}", type_name::<Literal>());
		// let mut trees = vec![];
		//
		// while tokens.has_next() {
		// 	let tree = tokens.parse::<TokenTree>()?;
		// 	println!(tree:?);
		// 	trees.push(tree.clone());
		// }
		//
		// Ok(Self {
		// 	data: TokenStream::from_iter(trees),
		// })
		Ok(Self {
			data: TokenStream::default(),
		})
	}

	fn transform(data: Self) -> TokenStream {
		data.data
	}
}

#[proc_macro]
pub fn cool_macro(
	token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let data: CoolMacro =
		Macro::parse(TokenStream::from(token_stream).into_iter().peekable())
			.expect(":(");
	Macro::transform(data).into()
}
