mod constants;
mod field;
pub mod goldilocks;
mod poseidon;
mod utils;

pub use poseidon::*;

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

#[wasm_bindgen]
// pub fn poseidon_u64(inputs: &[u64]) -> [u64; 4]
pub fn poseidon_u64_wrapper(inputs: &[u64]) -> Vec<JsValue> {
    let result = poseidon_u64(inputs);
    let mut result_js = Vec::with_capacity(4);

    for value in result.iter() {
        let bigint_value: u64 = *value;
        let bigint_string = format!("{}", bigint_value);
        result_js.push(JsValue::from_str(&bigint_string));
    }

    result_js
}

#[wasm_bindgen]
// pub fn poseidon_u64_bytes(bytes: &[u8]) -> [u64; 4]
pub fn poseidon_u64_bytes_wrapper(inputs: &[u8]) -> Vec<JsValue> {
    let result = poseidon_u64_bytes(inputs);
    let mut result_js = Vec::with_capacity(4);

    for value in result.iter() {
        let bigint_value: u64 = *value;
        let bigint_string = format!("{}", bigint_value);
        result_js.push(JsValue::from_str(&bigint_string));
    }

    result_js
}

#[wasm_bindgen]
//pub fn poseidon_u64_for_bytes(inputs: &[u64]) -> [u8; 32]
pub fn poseidon_u64_for_bytes_wrapper(inputs: &[u64]) -> JsValue {
    let result = poseidon_u64_for_bytes(inputs);
    let result_js = serde_wasm_bindgen::to_value(&result)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Error converting poseidon_u64_for_bytes result to JsValue: {:?}",
                e
            ))
        })
        .expect("REASON");
    result_js
}

#[wasm_bindgen]
// pub fn poseidon_u64_bytes_for_bytes(bytes: &[u8]) -> [u8; 32]
pub fn poseidon_u64_bytes_for_bytes_wrapper(inputs: &[u8]) -> JsValue {
    let result = poseidon_u64_bytes_for_bytes(inputs);
    let result_js = serde_wasm_bindgen::to_value(&result)
        .map_err(|e| {
            JsValue::from_str(&format!(
                "Error converting poseidon_u64_bytes_for_bytes result to JsValue: {:?}",
                e
            ))
        })
        .expect("REASON");
    result_js
}
