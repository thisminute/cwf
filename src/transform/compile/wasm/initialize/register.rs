use {data::semantics::Semantics, proc_macro2::TokenStream, quote::quote};

impl Semantics {
	pub fn static_register_all(&self, group_id: usize) -> TokenStream {
		let elements = self.static_register_elements(group_id);
		let classes = self.static_register_classes(group_id);
		let listeners = self.static_register_listeners(group_id);
		let properties = self.static_register_properties(group_id);
		quote! {
			#elements
			#classes
			#listeners
			#properties
		}
	}

	fn static_register_elements(&self, class_id: usize) -> TokenStream {
		self.groups[class_id]
			.elements
			.iter()
			.map(|&element_id| {
				let rules = self.static_register_all(element_id);
				quote! {
					class.elements.push({
						let mut class = Group::default();
						#rules
						class
					});
				}
			})
			.collect()
	}

	fn static_register_classes(&self, group_id: usize) -> TokenStream {
		self.groups[group_id]
			.classes
			.iter()
			.flat_map(|(_, groups)| groups.iter())
			.map(|&class_id| {
				let selector = self.groups[class_id]
					.selector
					.clone()
					.expect("static and dynamic classes should have selectors");
				let rules = self.static_register_all(class_id);
				quote! {
					class.classes.push({
						let mut class = Group::default();
						class.selector = #selector;
						#rules
						class
					});
				}
			})
			.collect()
	}

	fn static_register_listeners(&self, class_id: usize) -> TokenStream {
		self.groups[class_id]
			.listeners
			.iter()
			.map(|&listener_id| {
				let selector = self.groups[listener_id]
					.name
					.clone()
					.expect("listeners should have event names");
				let rules = self.static_register_all(listener_id);
				quote! {
					class.listeners.push({
						let mut class = Group::default();
						class.selector = #selector;
						#rules
						class
					});
				}
			})
			.collect()
	}

	fn static_register_properties(&self, class_id: usize) -> TokenStream {
		let css = self.groups[class_id]
			.properties
			.css
			.iter()
			.map(|(property, value)| {
				quote! {
					class.properties.insert(Property::Css(#property), #value);
				}
			});
		let cui = self.groups[class_id]
			.properties
			.cui
			.iter()
			.map(|(property, value)| {
				quote! {
					class.properties.insert(Property::#property, #value);
				}
			});
		quote! {
			#( #css )*
			#( #cui )*
		}
	}
}
