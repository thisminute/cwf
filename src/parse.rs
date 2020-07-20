use {
	crate::tokens::*,
	syn::{
		braced,
		export::Span,
		ext::IdentExt,
		parse::{Parse, ParseStream},
		token::Brace,
		Ident, Token,
	},
};

// can't implement Peek for some reason, so using these silly functions instead
fn peek_rule(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Token![:])
}
fn peek_block(input: ParseStream) -> bool {
	input.peek(Ident::peek_any) && input.peek2(Brace)
}
fn peek_prefixed_block(input: ParseStream) -> bool {
	(input.peek(Token![.]) || input.peek(Token![!]) || input.peek(Token![?]))
		&& input.peek2(Ident::peek_any)
		&& input.peek3(Brace)
}

impl Parse for HyphenatedIdent {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let mut parts = Vec::new();
		while input.peek(Ident::peek_any) {
			parts.push(input.parse()?);
			match input.parse::<Token![-]>() {
				Ok(_) => {
					continue;
				}
				Err(_) => {
					break;
				}
			}
		}

		Ok(Self { parts })
	}
}

impl Parse for Rule {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let property = input.parse()?;
		input.parse::<Token![:]>()?;
		let value = input.parse()?;
		input.parse::<Token![;]>()?;
		Ok(Self { property, value })
	}
}

impl Parse for Block {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let prefix = if input.peek(Token![.]) {
			input.parse::<Token![.]>()?;
			Prefix::Class
		} else if input.peek(Token![!]) {
			input.parse::<Token![!]>()?;
			Prefix::Action
		} else if input.peek(Token![?]) {
			input.parse::<Token![?]>()?;
			Prefix::Listener
		} else {
			Prefix::Instance
		};

		let identifier = input.parse()?;

		let content;
		braced!(content in input);

		let mut rules = Vec::new();
		let mut blocks = Vec::new();
		loop {
			if peek_rule(&content) {
				rules.push(content.parse()?);
			} else if peek_block(&content) || peek_prefixed_block(&content) {
				blocks.push(content.parse()?);
			} else {
				break;
			}
		}

		Ok(Self {
			prefix,
			identifier,
			rules,
			blocks,
		})
	}
}

impl Parse for Document {
	fn parse(input: ParseStream) -> Result<Self, syn::Error> {
		let mut rules = Vec::new();
		let mut blocks = Vec::new();
		loop {
			if peek_rule(&input) {
				rules.push(input.parse()?);
			} else if peek_block(&input) || peek_prefixed_block(&input) {
				blocks.push(input.parse()?);
			} else {
				break;
			}
		}

		Ok(Self {
			root: Block {
				identifier: Ident::new("thing", Span::call_site()),
				prefix: Prefix::Instance,
				rules,
				blocks,
			},
		})
	}
}