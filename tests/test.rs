// Test suite for Node
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use numeric_oxide::{oxidate, oxidate_multiple};
use std::assert_eq;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn oxidate_from_js() {
    assert_eq!(
        match oxidate("div(2,mod(sub(mult(4,add(1,1))1),5))".to_string()) {
            Ok(v) => v,
            Err(_) => panic!("Error"),
        },
        "1".to_string()
    );
}

#[wasm_bindgen_test]
fn oxidate_multiple_from_js() {
    match oxidate_multiple(
        JsValue::from_serde(&vec!["add(1,1)".to_string(), "add(2,2)".to_string()]).unwrap(),
    ) {
        Ok(v) => {
            let results: Vec<String> = JsValue::into_serde(&v).unwrap();
            assert_eq!(results, vec!["2".to_string(), "4".to_string()]);
        }
        Err(_) => panic!("Error"),
    }
}
