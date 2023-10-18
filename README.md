# mitzvah

### Noun

/ˈmɪtsvə/; Hebrew: מִצְוָה

1. ~~Any of the 613 commandments of Jewish law.~~
2. **An act of kindness, a good deed.**

An alternative to ~~`syn`~~ sin.

## Features

- [ ] Primitives
  - [x] `Ident`
  - [x] `Literal`
    - [x] `LiteralKind`
    - [x] `.suffix()`
    - [x] `.kind()`
  - [x] `Punct`
  - [x] `Group`
  - [x] `TokenTree`
- [ ] Helper Tokens
  - [ ] `Literal`Kinds
  - [x] `MultiPunct`
  - [ ] `Path`
  - [ ] `Expr`
  - [ ] `Fn`
- [ ] `trait Macro`
- [ ] Macro creation helper macros.

## Why?

I wanted to make my own opinionated library to create proc macros with 
while learning more about how the internals work without the abstractions 
of libraries.

`mitzvah` is made to be close to `proc_macro`; the main functionality only 
applies traits to `proc_macro`'s primitives and `TokenStream` to extend 
functionality to feel like `syn`.

To create a new token `impl Token` on a struct, and you'll be able to use 
it in the extended `TokenStream::parse::<CustomToken>()` function.

`mitzvah` also comes with some pre-built helper tokens such as all the 
`Literal`Kinds, `MultiPunct`, and `Path`.

## What did you learn?

### `proc_macro` is Lackluster

Rust has two places where it declares tokens.

1. The root of `proc_macro`.
2. Inside `proc_macro::bridge` (I'll call this `bridge` for brevity).

`bridge` contains all the useful data, while `proc_macro` is a wrapper 
over the tokens defined there, and it has the internal `bridge` private.

Let's take a look at token's definition in `proc_macro` compared to 
its definition in `bridge`.

```rs
// proc_macro

#[derive(Clone)]
pub struct Literal(
	// Notice this is private.
	bridge::Literal<bridge::client::Span, bridge::client::Symbol>
);

// proc_macro::bridge

// Notice the internal version has `Eq` and `PartialEq` while the wrapper 
// does not.
#[derive(Clone, Eq, PartialEq)]
pub struct Literal<Span, Symbol> {
	// The kind of literal such as `Str` and `Integer`.
	pub kind: LitKind,
	// The actual data such as `"1.0f64"`. 
	pub symbol: Symbol,
	// The suffix if there is one such as `"usize"`.
	pub suffix: Option<Symbol>,
	
	// This is the only thing that you can access from `proc_macro`.
	pub span: Span,
}
```

`bridge` has significantly more data--most of it being important parts--but 
`proc_macro` only exposes the `Span` from it.

This means if you want to only parse a token more specific than just 
`Literal` (like a string literal), you need to *reparse* data *from a 
string* that was <u>already parsed internally</u>. This is obviously 
both slower *and* more prone to bugs/inconsistencies.

`bridge` can actually be accessed through an unstable feature called 
`proc_macro_internals`, but since `proc_macro` is what gets passed to you 
and because there's no way to convert between the two, it's useless.

My conclusion is Rust's built-in `proc_macro` module lacks the information 
needed to effectively and safely build a macro without using external 
libraries.

What makes this extra disappointing is the data needed to solve this 
problem already exists, but it's hidden behind private fields and 
incomplete-feeling wrappers.

In a world where this glorious data is exposed, a library like `syn` could 
be reduced significantly to only include more complex tokens such as 
`Path` which aren't in `proc_macro` instead of having to re-implement all 
the primitives just to provide a decent developer experience.

Naturally, this sort of cut would make the library faster in both 
compiletime and runtime.

## What about testing?

While I'd love to have testing tokens well-supported, `proc_macro` doesn't 
support being run in non-{proc macro} crates.

To solve this, `mitzvah` includes the feature `proc-macro2` which will 
`impl Token` on the primitives there instead. It also marks `syn` and 
`proc-macro2` as `optional` dependencies, so it will only [download, 
compile, and use] them when the feature is set.

You can use the following in your `Config.toml` to automatically enable
`feature = "proc-macro2"` for tests, but not for your macros in the real 
world.
```toml
[dependencies]
mitzvah = ".."

[dev-dependencies]
mitzvah = { version = "..", features = ["proc-macro2"] }
```

The token primitives and more for the selected proc macro implementation are 
re-exported at `mitzvah::pm`, so you should use this to ensure testing 
works properly.

This is far from ideal--especially considering the differences between 
`proc_macro` and `proc-macro2`--but it enables the testing of tokens you 
create with `mitzvah`.

### Why not *just* use `proc-macro2`?

`proc-macro2` is awesome, but the point of `mitzvah` is to be an extension 
of `proc_macro` rather than a wrapper over it--hence only including 
extension traits on the primitives and brand-new more complex tokens.

`proc-macro2` actually uses 
[a runtime check](https://docs.rs/proc-macro2/1.0.69/src/proc_macro2/detection.rs.html#7-16) 
to determine whether it's running in a proc macro, while `mitzvah` 
determines its fallback at compiletime. It's likely optimized into atoms, 
but in the end `compiletime > runtime`.

With the way `mitzvah` works it could even provide its own feature-locked 
proc macro implementation, but that's way out of my scope.
