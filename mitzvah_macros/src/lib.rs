#[proc_macro_attribute]
pub fn r#macro(
	_attribute: proc_macro::TokenStream,
	token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	token_stream
}
