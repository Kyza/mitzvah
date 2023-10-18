use mitzvah::ext::literal::{LiteralKind, MitzvahLiteralExt};
use mitzvah_tests::cool_macro;
use std::str::FromStr;

// use mitzvah::pm::{LexError, Literal};

#[test]
fn cool_macro()
	// -> Result<(), LexError>
{
	// assert_eq!(Literal::from_str("1")?.kind(), LiteralKind::Integer);
	cool_macro!(1usize);
	// Ok(())
}
