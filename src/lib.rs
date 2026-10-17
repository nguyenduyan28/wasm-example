use wasm_bindgen::prelude::*;
// wasm-pack uses wasm-bindgen to bridge between JS and rust types
// allows JS to call Rust API with a string or function

#[wasm_bindgen] // wasm_bindgen is called an attribute, and it modifies the next statement
extern "C" {
    // that statement is extern, extern tell Rust that we want to call some externally defined function.
    pub fn alert(s: &str);
}
// the mark #[wasm_bindgen] will know how to find these functions
// the alert function here is the alert function of JS, we want to call this in Rust
// so set it with extern and #[wasm_bindgen] will set for us.

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
// we want a rust function to called in JS, it is opposite with extern