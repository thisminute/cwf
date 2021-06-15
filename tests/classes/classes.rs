extern crate cascading_wasm_language;
extern crate wasm_bindgen_test;
use self::{
	cascading_wasm_language::{cwl_header, cwl_test_setup},
	wasm_bindgen_test::*,
};

cwl_header!();

#[wasm_bindgen_test]
fn base() {
	cwl_test_setup! {
		.thingy {
			text: "hello world";
		}
		thingy {}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}

#[wasm_bindgen_test]
fn hoisting() {
	cwl_test_setup! {
		thingy {}
		.thingy {
			text: "hello world";
		}
	}
	assert_eq!(
		root.first_element_child()
			.expect("the root should contain an element")
			.inner_html(),
		"hello world"
	);
}