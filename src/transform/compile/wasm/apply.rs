use {
	data::semantics::{properties::CwlProperty, Semantics},
	proc_macro2::TokenStream,
	quote::quote,
	transform::compile::css::Css,
};

impl Semantics {
	fn apply_all(&self, group_id: usize) -> TokenStream {
		let classes = self.apply_classes(group_id);
		let listeners = self.apply_listeners(group_id);
		let elements = self.apply_elements(group_id);
		let properties = self.apply_properties(group_id);
		quote! {
			#classes
			#listeners
			#elements
			#properties
		}
	}

	pub fn apply_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let selector = self.groups[class_id]
					.selector
					.as_ref()
					.expect("dynamic classes should have a selector");
				let rules = self.apply_all(class_id);
				quote! {
					let elements = document.get_elements_by_class_name(#selector);
					for i in 0..elements.length() {
						let mut element = elements
							.item(i)
							.unwrap()
							.dyn_into::<HtmlElement>()
							.unwrap();
						#rules
					}
				}
			})
			.collect()
	}

	pub fn apply_listeners(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let rules = self.apply_all(listener_id);
				let event = match &**self.groups[listener_id]
					.name
					.as_ref()
					.expect("every listener should have an event id")
				{
					"click" => quote! { set_onclick },
					"mouseover" => quote! { set_onmouseover },
					_ => panic!("unknown event id"),
				};
				quote! {
					let closure = {
						let mut element = element.clone();
						Closure::wrap(Box::new(move |_e: Event| {
							let window = web_sys::window().expect("getting window");
							let document = window.document().expect("getting `window.document`");
							#rules
						}) as Box<dyn FnMut(Event)>)
					};
					element.#event(Some(closure.as_ref().unchecked_ref()));
					closure.forget();
				}
			})
			.collect()
	}

	fn apply_elements(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.elements
			.iter()
			.map(|&element_id| {
				let rules = self.apply_all(element_id);
				let tag = self.groups[element_id].tag();
				let class_names = &self.groups[element_id]
					.class_names
					.iter()
					.map(|class_name| quote! { #class_name, })
					.collect::<TokenStream>();
				quote! {
					element.append_child({
						let mut element = create_element(#tag, vec![#class_names]);
						#rules
						&element.into()
					}).unwrap();
				}
			})
			.collect()
	}

	fn apply_properties(&self, group_id: usize) -> TokenStream {
		let properties = &self.groups[group_id].properties;
		eprintln!("applying properties of group {}", group_id);
		let mut effects = Vec::new();
		if let Some(value) = properties.cwl.get(&CwlProperty::Text) {
			effects.push(quote! { element.text(#value); });
		}
		if let Some(_value) = properties.cwl.get(&CwlProperty::Link) {
			effects.push(quote! {});
		}

		for (property, value) in &properties.css {
			let property = property.css();
			effects.push(quote! { element.css(#property, #value); });
		}

		quote! { #( #effects )* }
	}
}